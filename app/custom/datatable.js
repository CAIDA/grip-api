let datatable = null;
let whois_dict = {};
let cidr_loose_re = /^[0-9]+[.:][0-9.:/]*$/;
const params = new Map(location.search.slice(1).split('&').map(kv => kv.split('=')))

function load_events_table(only_benign=false) {
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
                } else if (key === "tags") {
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
        if(only_benign){
            url += "&benign=true";
        }
        url = url.replace(/[?&]$/i, "");
        console.log(url);
        datatable = $('#datatable').DataTable({
                "processing": true,
                "serverSide": true,
                "searching": false,
                "ordering": false,
                "pageLength": 10,
                "ajax": {
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
                            let victims = extract_victims(data[0], row["event_type"]);
                            let links = "";
                            if(victims.length>0){
                                links = render_origin_links(victims.slice(0,2), false, row['external']);
                                if(victims.length>1){
                                    links+= `<div>(${victims.length-1} more)</div>`
                                }
                            }
                            return links;
                        },
                        "targets": [0]
                    },
                    {
                        "render": function (data, type, row) {
                            let attackers = extract_attackers(data[0], row["event_type"]);
                            if(attackers ===null){
                                return ""
                            }
                            let links = "";
                            if(attackers.length>0){
                                links = render_origin_links(attackers.slice(0,2), false, row['external']);
                                if(attackers.length>1){
                                    links+= `<div>(${attackers.length-1} more)</div>`
                                }
                            }
                            return links;
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
                                let start_ts = Date.parse(row["view_ts"]);
                                let end_ts = Date.parse(data);
                                let duration = (end_ts-start_ts)/1000/60;
                                if(duration < 0 ){
                                    // FIXME: figure out why duration would be below 0
                                    duration = 0;
                                }
                                return `${duration} min`
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
        $('#datatable').on( 'processing.dt', function () {
            console.log("processing");
            let page_number = datatable.page.info()['page'];
            if(window.location.hash){
                let hash_number = parseInt(window.location.hash.split("#")[1]);
                if(page_number!==hash_number){
                    console.log(`change page now from ${page_number} to ${hash_number}`);
                    datatable.page(hash_number).draw(false);
                }
            }
        });
        $('#datatable').on( 'page.dt', function () {
            let info = datatable.page.info();
            window.location.hash = info['page'];
            console.log(info)
            datatable.draw(false);
        } );

        $('#datatable tbody').on('click', 'tr', function (e) {
            if(e.target.tagName === 'A'){
                return;
            }
            var data = datatable.row($(this)).data();
            let base = window.location.pathname.split("/")[1];
            if(base === "events_benign"){
                base = "events"
            }
            window.open(`/${base}/` + data['event_type'] + "/" + data['id'], '_self', false);
        });
    });

    $("#range-btn").click(function () {
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
        let tags = [];
        let ready = false;
        console.log(fields);
        for(let i in fields){
            let v = fields[i].trim().toLowerCase();
            // check if it's a prefix
            if(cidr_loose_re.test(v)){
                prefix = v;
                ready = true;
            }

            // check if it's a tag
            if((/^[a-zA-Z\-]+$/).test(v)){
                tags.push(v);
                ready = true;
            }

            // check if it's an as number
            v = v.replace(/as/i,"");
            if((/^[0-9]+$/).test(v)){
                // it's a asn
                asn = v;
                ready = true;
            }
        }
        if(!ready){
            // alert("not enough search parameters");
            console.log("not enough search parameters");
            return;
        }
        let url = `/${window.location.pathname.split("/")[1]}/${event_type}?`;
        if(prefix!==""){
            url+=`prefix=${prefix}&`;
        }
        if(asn!==""){
            url+=`asn=${asn}&`;
        }
        if(tags.length>0){
            url+=`tags=${tags.join(",")}`
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
                console.log(event);
                render_event_details_table(event_type, event);
                render_pfx_event_table(event_type, event['pfx_events'], event['tr_metrics']['tr_skipped'], event['tr_metrics']['tr_skip_reason']);
            }
        });
    })
}

function load_blacklist(){
    let blacklist = [];
    $.ajax({
        dataType: "json",
        async: false,
        url: "/json/blacklist",
        success: function (data) {
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
                    return render_origin_links( [data], true);
                },
                "targets": [0]
            }
        ]
    })
}

function load_tags(){
    let tags = [];
    let tr = [];
    $.ajax({
        dataType: "json",
        async: false,
        url: "/json/tags",
        success: function (data) {
            for(let tag in data['definitions']){
                let d =  data['definitions'][tag];
                tags.push([tag, d["definition"]])
            }

            for(let entry of data['tr_worthy']){
                tr.push([entry['tags'], entry['worthy'], entry["explain"], entry["apply_to"]])
            }
        }
    });

    console.log(tr)

    $('#tags').DataTable({
        searching: false,
        ordering: false,
        paging: false,
        data:tags,
        columns: [
            {title: "Tag ID"},
            {title: "Definition"},
        ],
        columnDefs: [
            // {
            //     "render": function (data, type, row) {
            //         // return render_origin_links( [data], style=2);
            //         let links = "";
            //         for(c of data){
            //             links+=`<div>${c}</div>\n`
            //         }
            //         console.log(data)
            //         return links
            //     },
            //     "targets": [2]
            // }
        ]
    });

    $('#tr_worthy').DataTable({
        searching: false,
        ordering: false,
        paging: false,
        data:tr,
        columns: [
            {title: "Tags"},
            {title: "TR Worthy"},
            {title: "Explain"},
            {title: "Apply To"},
        ],
        columnDefs: [
            // {
            //     "render": function (data, type, row) {
            //         // return render_origin_links( [data], style=2);
            //         let links = "";
            //         for(c of data){
            //             links+=`<div>${c}</div>\n`
            //         }
            //         console.log(data)
            //         return links
            //     },
            //     "targets": [2]
            // }
        ]
    })
}
