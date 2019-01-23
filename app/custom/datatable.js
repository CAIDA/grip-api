let datatable = null;
let whois_dict = {};
let cidr_loose_re = /^[0-9]+[.:][0-9.:/]*$/;
const params = new Map(location.search.slice(1).split('&').map(kv => kv.split('=')))

function load_events_table(event_type) {
    $.extend(true, $.fn.dataTable.defaults, {
        "searching": false,
        "ordering": false,
    });
    $(document).ready(function () {
        let url = `/json/events/${event_type}?`;
        let search_text = [];
        if(!params.has("")){
            params.forEach(function(value, key, map){
                url += `${key}=${value}&`;
                if(key === "asn"){
                    search_text.push("AS"+value)
                } else if (key === "prefix") {
                    search_text.push(value)
                }
            });
        }
        $("#search-box").val(search_text.join(" "));
        url = url.replace(/[?&]$/i, "");
        console.log(url);
        datatable = $('#datatable').DataTable({
                "processing": true,
                "serverSide": true,
                "searching": false,
                "ordering": false,
                "ajax": {
                    // "url": `/json/events/${event_type}`,
                    "url": url,
                },
                "columns": [
                    {title: "Poential Victim", "data": 'pfx_events'},
                    {title: "Poential Attacker", "data": 'pfx_events'},
                    {title: "Largest Prefix", "data": 'pfx_events'},
                    {title: "# Prefix Events", "data": 'pfx_events'},
                    {title: "Start Time", "data": 'view_ts'},
                    {title: "Duration", "data": 'finished_ts'},
                    {title: "Type", "data": 'view_ts'},
                ],
                "columnDefs": [
                    {
                        "render": function (data, type, row) {
                            let pfxevent = data[0];
                            switch(row["event_type"]){
                                case "moas":
                                    let oldcomers = new Set();
                                    for(let i in pfxevent["origins"]){
                                        oldcomers.add(pfxevent['origins'][i]);
                                    }
                                    for(let i in pfxevent["newcomer_origins"]){
                                        oldcomers.delete(pfxevent['newcomer_origins'][i])
                                    }
                                    return [...oldcomers].join(" ");
                                case "submoas":
                                    return pfxevent["sub_origins"].join(" ");
                                case "defcon":
                                    return pfxevent["origins"].join(" ");
                                case "edges":
                                    return [pfxevent["as1"], pfxevent["as2"]].join(" ");
                                default:
                                    return "wrong"
                            }
                        },
                        "targets": [0]
                    },
                    {
                        "render": function (data, type, row) {
                            let pfxevent = data[0];
                            switch(row["event_type"]){
                                case "moas":
                                    return pfxevent["newcomer_origins"].join(" ");
                                case "submoas":
                                    return pfxevent["super_origins"].join(" ");
                                case "defcon":
                                    return "N/A";
                                case "edges":
                                    return [pfxevent["as1"], pfxevent["as2"]].join(" ");
                                default:
                                    return "wrong"
                            }
                        },
                        "targets": [1]
                    },
                    {
                        "render": function (data, type, row) {
                            let largest_pfx_len = 1000;
                            let largest_pfx = "";
                            for(let i in data){
                                let pfxevent = data[i];
                                let p = "";
                                if("prefix" in pfxevent){
                                    p = pfxevent["prefix"];
                                } else {
                                    p = pfxevent["sub_pfx"];
                                }
                                let len = parseInt(p.split("/")[1])
                                if(len <= largest_pfx_len){
                                    largest_pfx = p;
                                    largest_pfx_len = len;
                                }
                            }
                            return largest_pfx;
                        },
                        "targets": [2]
                    },
                    {
                        "render": function (data, type, row) {
                            let num_pfx = 0;
                            let num_addrs = 0;
                            for(let i in data){
                                num_pfx++;
                                let pfxevent = data[i];
                                let p = "";
                                if("prefix" in pfxevent){
                                    p = pfxevent["prefix"];
                                } else {
                                    p = pfxevent["sub_pfx"];
                                }
                                let len = parseInt(p.split("/")[1]);
                                if(len<=32){
                                    num_addrs += Math.pow(2, 32-len);
                                } else {
                                    num_addrs += Math.pow(2, 128-len);
                                    console.log(len, num_addrs)
                                }
                            }
                            if(num_addrs.toString().includes("e")){
                                num_addrs = num_addrs.toPrecision(2)
                            }
                            return `${num_pfx} pfxs ${num_addrs} addresses`
                        },
                        "targets": [3]
                    },
                    {
                        "render": function (data, type, row) {
                            return data.split("T").join("  ")
                        },
                        "targets": [4]
                    },
                    {
                        "render": function (data, type, row) {
                            if (data === null) {
                                return "On-Going"
                            } else {
                                start_ts = Date.parse(row["view_ts"]);
                                end_ts = Date.parse(data);
                                return `${(end_ts-start_ts)/1000/60} min`
                            }
                        },
                        "targets": [5]
                    },
                    {
                        "render": function (data, type, row) {
                            switch( row["event_type"]){
                                case 'moas':
                                    return "origin hijack (moas)";
                                case 'submoas':
                                    return "origin hijack (submoas)";
                                case 'edges':
                                    return "path manipulation (new edge)";
                                case 'defcon':
                                    return "path manipulation (defcon)";
                            }
                        },
                        "targets": [6]
                    },
                ]
            }
        );

        $('#datatable tbody').on('click', 'tr', function () {
            var data = datatable.row($(this)).data();
            console.log("/json/event/id/" + data['id']);
            window.open("/event/" + data['event_type'] + "/" + data['id'], '_self', false);
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
        if(!ready){
            alert("not enough search parameters");
            return;
        }
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
        const event_id = get_event_id_from_url();
        const event_type = get_event_type_from_url();
        load_event_scripts();

        $.ajax({
            url: "/json/event/id/" + event_id,
            success: function (event) {
                render_pfx_event_table(event_type, event["data"]);
            }
        });
    })
}
