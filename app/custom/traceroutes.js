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
colors = [
"#63b598", "#ce7d78", "#ea9e70", "#a48a9e", "#c6e1e8", "#648177" ,"#0d5ac1" ,
"#f205e6" ,"#1c0365" ,"#14a9ad" ,"#4ca2f9" ,"#a4e43f" ,"#d298e2" ,"#6119d0",
"#d2737d" ,"#c0a43c" ,"#f2510e" ,"#651be6" ,"#79806e" ,"#61da5e" ,"#cd2f00" ,
"#9348af" ,"#01ac53" ,"#c5a4fb" ,"#996635","#b11573" ,"#4bb473" ,"#75d89e" ,
"#2f3f94" ,"#2f7b99" ,"#da967d" ,"#34891f" ,"#b0d87b" ,"#ca4751" ,"#7e50a8" ,
"#c4d647" ,"#e0eeb8" ,"#11dec1" ,"#289812" ,"#566ca0" ,"#ffdbe1" ,"#2f1179" ,
"#935b6d" ,"#916988" ,"#513d98" ,"#aead3a", "#9e6d71", "#4b5bdc", "#0cd36d",
"#250662", "#cb5bea", "#228916", "#ac3e1b", "#df514a", "#539397", "#880977",
"#f697c1", "#ba96ce", "#679c9d", "#c6c42c", "#5d2c52", "#48b41b", "#e1cf3b",
"#5be4f0", "#57c4d8", "#a4d17a", "#225b8", "#be608b", "#96b00c", "#088baf",
"#f158bf", "#e145ba", "#ee91e3", "#05d371", "#5426e0", "#4834d0", "#802234",
"#6749e8", "#0971f0", "#8fb413", "#b2b4f0", "#c3c89d", "#c9a941", "#41d158",
"#fb21a3", "#51aed9", "#5bb32d", "#807fb", "#21538e", "#89d534", "#d36647",
"#7fb411", "#0023b8", "#3b8c2a", "#986b53", "#f50422", "#983f7a", "#ea24a3",
"#79352c", "#521250", "#c79ed2", "#d6dd92", "#e33e52", "#b2be57", "#fa06ec",
"#1bb699", "#6b2e5f", "#64820f", "#1c271", "#21538e", "#89d534", "#d36647",
"#7fb411", "#0023b8", "#3b8c2a", "#986b53", "#f50422", "#983f7a", "#ea24a3",
"#79352c", "#521250", "#c79ed2", "#d6dd92", "#e33e52", "#b2be57", "#fa06ec",
"#1bb699", "#6b2e5f", "#64820f", "#1c271", "#9cb64a", "#996c48", "#9ab9b7",
"#06e052", "#e3a481", "#0eb621", "#fc458e", "#b2db15", "#aa226d", "#792ed8",
"#73872a", "#520d3a", "#cefcb8", "#a5b3d9", "#7d1d85", "#c4fd57", "#f1ae16",
"#8fe22a", "#ef6e3c", "#243eeb", "#1dc18", "#dd93fd", "#3f8473", "#e7dbce",
"#421f79", "#7a3d93", "#635f6d", "#93f2d7", "#9b5c2a", "#15b9ee", "#0f5997",
"#409188", "#911e20", "#1350ce", "#10e5b1", "#fff4d7", "#cb2582", "#ce00be",
"#32d5d6", "#17232", "#608572", "#c79bc2", "#00f87c", "#77772a", "#6995ba",
"#fc6b57", "#f07815", "#8fd883", "#060e27", "#96e591", "#21d52e", "#d00043",
"#b47162", "#1ec227", "#4f0f6f", "#1d1d58", "#947002", "#bde052", "#e08c56",
"#28fcfd", "#bb09b", "#36486a", "#d02e29", "#1ae6db", "#3e464c", "#a84a8f",
"#911e7e", "#3f16d9", "#0f525f", "#ac7c0a", "#b4c086", "#c9d730", "#30cc49",
"#3d6751", "#fb4c03", "#640fc1", "#62c03e", "#d3493a", "#88aa0b", "#406df9",
"#615af0", "#4be47", "#2a3434", "#4a543f", "#79bca0", "#a8b8d4", "#00efd4",
"#7ad236", "#7260d8", "#1deaa7", "#06f43a", "#823c59", "#e3d94c", "#dc1c06",
"#f53b2a", "#b46238", "#2dfff6", "#a82b89", "#1a8011", "#436a9f", "#1a806a",
"#4cf09d", "#c188a2", "#67eb4b", "#b308d3", "#fc7e41", "#af3101", "#ff065",
"#71b1f4", "#a2f8a5", "#e23dd0", "#d3486d", "#00f7f9", "#474893", "#3cec35",
"#1c65cb", "#5d1d0c", "#2d7d2a", "#ff3420", "#5cdd87", "#a259a4", "#e4ac44",
"#1bede6", "#8798a4", "#d7790f", "#b2c24f", "#de73c2", "#d70a9c", "#25b67",
"#88e9b8", "#c2b0e2", "#86e98f", "#ae90e2", "#1a806b", "#436a9e", "#0ec0ff",
"#f812b3", "#b17fc9", "#8d6c2f", "#d3277a", "#2ca1ae", "#9685eb", "#8a96c6",
"#dba2e6", "#76fc1b", "#608fa4", "#20f6ba", "#07d7f6", "#dce77a", "#77ecca"]

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

    line_counter = 0
    measurements.forEach(function(measurement){
        measurement["results"].forEach(function(result){
            let nodes = []
            console.log(result["hops"])
            for(let key in result["hops"]){
                hop = result["hops"][key]
                if("lat" in hop && hop["lat"]!==0){
                    nodes.push([hop["lat"], hop["long"]])
                }
            }


            if(nodes.length>0) {
                L.polyline(nodes, {
                    weight: 1,
                    color: colors[line_counter],
                    dashArray: '2, 2'
                })
                    .addTo(map)
                    .setText('\u2708     ', {
                        repeat: true,
                        offset: 8,
                        attributes: {
                            'font-weight': 'bold',
                            'font-size': '24'
                        }
                    });

                line_counter++;
            }
        })
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
                    measurements.push(row);
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
