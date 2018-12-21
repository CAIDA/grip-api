function load_pfx_event() {
    $(document).ready(function () {
        let path = window.location.pathname.replace(/\/$/, "");
        let path_segments = path.split("/");
        let event_id = path_segments[path_segments.length - 2];
        let pfx_fingerprint = path_segments[path_segments.length - 1];

        $.ajax({
            url: `/json/pfx_event/id/${event_id}/${pfx_fingerprint}`,
            success: function (pfx_event) {
                let download_path = event_id + "-" + pfx_fingerprint + ".json";
                draw_json_raw(JSON.stringify(pfx_event, undefined, 4), download_path);
                draw_pfx_event_table(pfx_event);
                let measurements = draw_traceroute_table(pfx_event);
                draw_sankeys(pfx_event);
                draw_traceroute_vis(measurements);
            }
        });
    });
}

function draw_sankeys(pfx_event){
    draw_monitor_sankey(pfx_event);
    draw_tr_sankey(pfx_event);
}

function draw_traceroute_vis(measurements) {
    if (measurements === undefined || measurements.length === 0) {
        let map = $("#traceroute-map");
        map.html("data not available");
        map.css('height', '100px');
        return
    }

    let map = new L.Map('traceroute-map');

    // create the tile layer with correct attribution
    var osmUrl = 'https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png';
    var osmAttrib = 'Map data © <a href="https://openstreetmap.org">OpenStreetMap</a> contributors';
    var osm = new L.TileLayer(osmUrl, {minZoom: 1, maxZoom: 18, attribution: osmAttrib});

    // start the map in South-East England
    map.setView(new L.LatLng(51.3, 0.7), 2);
    map.addLayer(osm);

    var plane = L.polyline([[-49.38237, -37.26562], [-1.75754, -14.41406], [51.61802, -23.20312]], {
        weight: 1,
        color: 'black',
        dashArray: '2, 2'
    }).addTo(map);
    plane.setText('\u2708     ', {
        repeat: true,
        offset: 8,
        attributes: {
            'font-weight': 'bold',
            'font-size': '24'
        }
    });
    console.log("map loaded")
}

function draw_pfx_event_table(pfx_event){
    render_pfx_event_table(get_event_type_from_url(), [pfx_event], "#pfx_event_table", false)

}

function draw_json_raw(json_raw_str, download_path) {
    $("#json_modal").html(syntaxHighlight(json_raw_str));
    $(".pfx-event-modal-download").click(function () {
        let dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(json_raw_str);
        var dlAnchorElem = document.getElementById('downloadAnchorElem');
        dlAnchorElem.setAttribute("href", dataStr);
        dlAnchorElem.setAttribute("download", download_path);
        dlAnchorElem.click();
    });
}

function draw_traceroute_table(pfx_event) {
    let measurements = [];
    $('#traceroutes_table').DataTable({
        data: pfx_event["traceroutes"],
        columns: [
            {title: "Measurement ID", data: "msm_id"},
            {title: "Target ASN", data: "target_asn"},
            {title: "Target IP", data: "target_ip"},
            {title: "Target Prefix", data: "target_pfx"},
            {title: "Results (from RIPE)", data: ""},
        ],
        paging: false,
        searching: false,
        "columnDefs": [
            {
                "render": function (data, type, row) {
                    load_origin_asrank(data);
                    return `<a class="btn btn-default as-btn as-btn-${data}" data-toggle="tooltip" title="" data-placement="top" href='http://as-rank.caida.org/asns/${data}' target="_blank")> AS${data} </a>`
                    // return `<button class="origin-button" onclick="window.open('http://as-rank.caida.org/asns/${data}')"> ${data} </button>`
                },
                "targets": [1]
            },
            {
                "render": function (data, type, row) {
                    let msm_id = row['msm_id'];
                    measurements.push(msm_id);
                    return `<button class="origin-button" onclick="window.open('https://atlas.ripe.net/measurements/${msm_id}/')"> general </button>` +
                        `<button class="origin-button" onclick="window.open('https://atlas.ripe.net/measurements/${msm_id}/#!openipmap')"> IP map </button>` +
                        `<button class="origin-button" onclick="window.open('https://atlas.ripe.net/api/v2/measurements/${msm_id}/results/?format=json')"> JSON </button>`
                },
                "targets": [4]
            },
        ]
    });
    return measurements
}

function extract_sankey_data(path_lst, space_separated = true) {
    path_count_dict = {};

    path_lst.forEach(function (path) {
        let asns = [];
        if (space_separated) {
            path = path.trim().replace(/ {2}/g, ' ');
            asns = path.split(" ");
        } else {
            asns = path.split(";");
        }
        if (asns.length > 1) {
            for (let i = 0; i < asns.length - 1; i++) {
                if (asns[i] === asns[i + 1]) {
                    continue
                }
                let segment = `${asns[i]},${asns[i + 1]}`;
                if (asns[i] === " " || asns[i + 1] === " ") {
                    alert(`${path} => ${asns} => ${i}: "${asns[i]}" "${asns[i + 1]}"`)
                }
                if (!(segment in path_count_dict)) {
                    path_count_dict[segment] = 0
                }
                path_count_dict[segment]++
            }
        }
    });

    let data = [];
    for (let key in path_count_dict) {
        let nodes = key.split(",");
        data.push([nodes[0], nodes[1], path_count_dict[key]])
    }

    console.log(data);
    return data
}

function draw_monitor_sankey(pfx_event) {
    google.charts.load('current', {'packages': ['sankey']});
    google.charts.setOnLoadCallback(drawChart);

    let path_data = [];
    if ("aspaths" in pfx_event) {
        path_data = extract_sankey_data(pfx_event["aspaths"], true)
    } else if ("super_aspaths" in pfx_event) {
        path_data = extract_sankey_data(pfx_event["super_aspaths"], true)
    } else {
        alert("no paths data available")
    }

    function drawChart() {
        var data = new google.visualization.DataTable();
        data.addColumn('string', 'From');
        data.addColumn('string', 'To');
        data.addColumn('number', 'Weight');
        data.addRows(path_data);

        // Sets chart options.
        var options = {
            width: 1200,
            height: data.getNumberOfRows() * 11 + 30
        };

        // Instantiates and draws our chart, passing in some options.
        var chart = new google.visualization.Sankey(document.getElementById('monitor_sankey_diagram'));
        chart.draw(data, options);
    }
}

function draw_tr_sankey(pfx_event) {

    let traceroutes = pfx_event["traceroutes"];
    let as_routes = [];
    traceroutes.forEach(function (traceroute) {
        if ("results" in traceroute) {
            traceroute["results"].forEach(function (result) {
                as_routes.push(
                    result["as_traceroute"].filter(asn => asn !== "*").join(";")
                );
            });
        }
    });

    let path_data = extract_sankey_data(as_routes, false);

    if (path_data.length === 0) {
        $("#tr_sankey_diagram").html("No data available");
    } else {


        google.charts.load('current', {'packages': ['sankey']});
        google.charts.setOnLoadCallback(drawChart);

        function drawChart() {
            var data = new google.visualization.DataTable();
            data.addColumn('string', 'From');
            data.addColumn('string', 'To');
            data.addColumn('number', 'Weight');
            data.addRows(path_data);

            // Sets chart options.
            var options = {
                width: 1200,
                height: data.getNumberOfRows() * 11 + 30
            };

            // Instantiates and draws our chart, passing in some options.
            var chart = new google.visualization.Sankey(document.getElementById('tr_sankey_diagram'));
            chart.draw(data, options);
        }

    }
}
