let datatable = null;
let whois_dict = {};
let cidr_loose_re = /^[0-9]+[.:][0-9.:/]*$/;
const params = new Map(location.search.slice(1).split('&').map(kv => kv.split('=')))

function load_events_table() {
    const event_type = get_event_type_from_url();
    let frame_type = event_type;
    if(frame_type==="all"){
        frame_type = "overall"
    }
    $.extend(true, $.fn.dataTable.defaults, {
        "searching": false,
        "ordering": false,
    });
    $(document).ready(function () {
        $('body').tooltip({selector: '[data-toggle="tooltip"]'});
        $("#stats-frame").html(`<iframe src="//ioda.caida.org/public/hijacks-trworthy-${frame_type}" width="100%" height="500" frameborder="0"></iframe>`);
        let url = `/json/events/${event_type}?`;
        let search_text = [];
        let start_ts = "";
        let end_ts = "";
        if(!params.has("")){
            params.forEach(function(value, key, map){
                if(!key.startsWith("ts_")) {
                    // strip existing searching ranges
                    url += `${key}=${value}&`;
                }else{
                    if(key==="ts_start"){
                        start_ts = value
                    } else if(key==="ts_end"){
                        end_ts = value
                    }
                }
                if(key === "asn"){
                    search_text.push("AS"+value)
                } else if (key === "prefix") {
                    search_text.push(value)
                }
            });
        }
        if(start_ts!==""){
            $('#reportrange span').html(start_ts + ' - ' + end_ts);
        }
        $("#search-box").val(search_text.join(" "));
        let times = $('#reportrange span').html().split(" - ");
        if(Date.parse(times[0]) !==null){
            url += `ts_start=${times[0]}&ts_end=${times[1]}`;
        }
        url = url.replace(/[?&]$/i, "");
        console.log(url);
        datatable = $('#datatable').DataTable({
                "processing": true,
                "serverSide": true,
                "searching": false,
                "ordering": false,
                "pageLength": 25,
                "ajax": {
                    // "url": `/json/events/${event_type}`,
                    "url": url,
                },
                "columns": [
                    {title: "Potential Victim", "data": 'pfx_events'},
                    {title: "Potential Attacker", "data": 'pfx_events'},
                    {title: "Largest Prefix", "data": 'pfx_events'},
                    {title: "# Prefix Events", "data": 'pfx_events'},
                    {title: "Start Time", "data": 'view_ts'},
                    {title: "Duration", "data": 'finished_ts'},
                    {title: "Type", "data": 'view_ts'},
                ],
                "columnDefs": [
                    {
                        "render": function (data, type, row) {
                            return render_origin_links( extract_victims(data[0], row["event_type"]));
                        },
                        "targets": [0]
                    },
                    {
                        "render": function (data, type, row) {
                            return render_origin_links( extract_attackers(data[0], row["event_type"]))
                        },
                        "targets": [1]
                    },
                    {
                        "width": "8em",
                        "render": function (data, type, row) {
                            return extract_largest_prefix(data)
                        },
                        "targets": [2]
                    },
                    {
                        "width": "12em",
                        "render": function (data, type, row) {
                            [num_pfx, num_addrs] = extract_impact(data);
                            return render_impact(num_pfx, num_addrs)
                        },
                        "targets": [3]
                    },
                    {
                        "width": "10em",
                        "render": function (data, type, row) {
                            return data.split("T").join("  ")
                        },
                        "targets": [4]
                    },
                    {
                        "width": "6em",
                        "render": function (data, type, row) {
                            if (data === null) {
                                return "ongoing"
                            } else {
                                start_ts = Date.parse(row["view_ts"]);
                                end_ts = Date.parse(data);
                                return `${(end_ts-start_ts)/1000/60} min`
                            }
                        },
                        "targets": [5]
                    },
                    {
                        "width": "13em",
                        "render": function (data, type, row) {
                            return event_type_explain[row["event_type"]];
                        },
                        "targets": [6]
                    },
                ]
            }
        );

        $('#datatable tbody').on('click', 'tr', function (e) {
            if(e.target.tagName === 'A'){
                return;
            }
            var data = datatable.row($(this)).data();
            window.open("/events/" + data['event_type'] + "/" + data['id'], '_self', false);
        });
    });

    $("#range-btn").click(function () {
        // let event_type = window.location.pathname.replace(/\/$/, "").split("/").pop();
        // let times = $('#reportrange span').html().split(" - ");
        // let url = `/json/events/${event_type}?ts_start=${times[0]}&ts_end=${times[1]}`;

        let url = window.location.pathname.replace(/\?.*\/$/, "");
        url+="?";
        if(!params.has("")){
            params.forEach(function(value, key, map){
                if(!key.startsWith("ts_")) {
                    // strip existing searching ranges
                    url += `${key}=${value}&`;
                }
            });
        }
        let times = $('#reportrange span').html().split(" - ");
        if(Date.parse(times[0]) !==null){
            url += `ts_start=${times[0]}&ts_end=${times[1]}`;
        }
        url = url.replace(/[?&]$/i, "");
        console.log(url);
        window.open(url, '_self', false);
    });

    $("#search-btn").click(function () {
        let fields = $("#search-box").val().trim().split(" ");
        if(fields.length > 2){
            alert("allow maximum searching for 1 ASN and 1 prefix")
            return;
        }
        let asn = "";
        let prefix = "";
        let ready = false;
        for(let i in fields){
            let v = fields[i].trim();
            if(cidr_loose_re.test(v)){
                // it's a prefix
                prefix = v;
                ready = true;
            }
            v = v.replace(/as/i,"");
            if((/^[0-9]+$/).test(v)){
                // it's a asn
                asn = v;
                ready = true;
            }
        }
        // if(!ready){
        //     alert("not enough search parameters");
        //     return;
        // }
        let url = `/events/${event_type}?`;
        if(prefix!==""){
            url+=`prefix=${prefix}&`;
        }
        if(asn!==""){
            url+=`asn=${asn}&`;
        }
        url = url.replace(/[?&]$/i, "");
        window.open(url, '_self', false);
    });


    $("#search-box").keyup(function(event) {
        if (event.keyCode === 13) {
            $("#search-btn").click();
        }
    });
}

var traceroute_hash = {};

function load_traceroute_page(uuid) {
    let pfx_event = traceroute_hash[uuid];
    let path = window.location.pathname.replace(/\/$/, "");
    let path_segments = path.split("/");
    let event_type = path_segments[path_segments.length - 2];

    let fingerprint = extract_pfx_event_fingerprint(pfx_event, event_type);
    window.open(`${path}/${fingerprint}`, "_self", false)
}

function extract_pfx_event_fingerprint(pfx_event, event_type) {
    let fingerprint = "";
    switch (event_type) {
        case "moas":
            fingerprint = `${pfx_event["prefix"]}`;
            break;
        case "submoas":
            fingerprint = `${pfx_event["sub_pfx"]}_${pfx_event["super_pfx"]}`;
            break;
        case "edges":
            fingerprint = `${pfx_event["prefix"]}`;
            break;
        case "defcon":
            fingerprint = `${pfx_event["sub_pfx"]}_${pfx_event["super_pfx"]}`;
            break;
        default:
            alert(`wrong event type ${event_type}`)
    }

    return fingerprint.replace(/\//g, "-")
}

function load_event_details() {
    $(document).ready(function () {
        $('body').tooltip({selector: '[data-toggle="tooltip"]'});
        const event_id = get_event_id_from_url();
        const event_type = get_event_type_from_url();
        load_event_scripts();

        $.ajax({
            url: "/json/event/id/" + event_id,
            success: function (event) {
                render_event_details_table(event_type, event);
                render_pfx_event_table(event_type, event["pfx_events"]);
            }
        });
    })
}

function load_blacklist(){
    let blacklist = []
    $.ajax({
        dataType: "json",
        async: false,
        url: "/json/blacklist",
        success: function (data) {
            data = JSON.parse(data);
            for(asn of data['blacklist']){
                blacklist.push([asn])
            }
        }
    });
    datatable = $('#datatable').DataTable({
        searching: false,
        ordering: false,
        paging: false,
        data:blacklist,
        columns: [
            {title: "Blacklist AS"},
        ],
        columnDefs: [
            {
                "render": function (data, type, row) {
                    return render_origin_links( [data], style=2);
                },
                "targets": [0]
            }
        ]
    })
}