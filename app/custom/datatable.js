let datatable = null;
let whois_dict = {};

function load_events_table(event_type) {
    $.extend(true, $.fn.dataTable.defaults, {
        "searching": false,
        "ordering": false,
    });
    $(document).ready(function () {

        datatable = $('#datatable').DataTable({
                "processing": true,
                "serverSide": true,
                "searching": false,
                "ordering": false,
                "ajax": {
                    "url": `/json/events/${event_type}`,
                },
                "columns": [
                    {title: "Event Type", "data": 'event_type'},
                    {title: "Fingerprint", "data": 'fingerprint'},
                    // {title: "Event ID", "data": 'id'},
                    {title: "Prefix Events", "data": 'pfx_events_cnt'},
                    {title: "Status", "data": 'finished_ts'},
                    {title: "Time Stamp", "data": 'view_ts'},
                ],
                "columnDefs": [
                    {
                        "render": function (data, type, row) {
                            if(data===null){
                                return "On-Going"
                            } else {
                                return `Finished at ${data}`
                            }
                        },
                        "targets": [3]
                    },
                ]
            }
        );

        $('#datatable tbody').on('click', 'tr', function () {
            var data = datatable.row($(this)).data();
            console.log("/json/event/id/" + data['id']);
            window.open("/event/" + data['event_type'] + "/" + data['id'], "_blank");
        });
    });

    $("#query-btn").click(function () {
        let event_type = window.location.pathname.replace(/\/$/, "").split("/").pop();
        let times = $('#reportrange span').html().split(" - ");
        let url = `/json/events/${event_type}?ts_start=${times[0]}&ts_end=${times[1]}`

        console.log(url);
        datatable.ajax.url(url).load();

    });
}

function guid() {
    function s4() {
        return Math.floor((1 + Math.random()) * 0x10000)
            .toString(16)
            .substring(1);
    }

    return s4() + s4() + '-' + s4() + '-' + s4() + '-' + s4() + '-' + s4() + s4() + s4();
}

var traceroute_hash = {};

function load_ripe_data(prefix, prefix_class) {
    $.ajax({
        url: `https://stat.ripe.net/data/prefix-overview/data.json?resource=${prefix}`,
        success: function (pfx_info) {
            let asns = pfx_info["data"]["asns"].map(function (elem) {
                return "AS" + elem["asn"]
            }).join(" ");
            $(`.pfx-btn-${prefix_class}`).html(`${prefix} ${asns}`)
        }
    });
}


function load_who_is(prefix) {
    if (!(prefix in whois_dict)){
        whois_dict[prefix] = "";
        $.ajax({
            url: `https://stat.ripe.net/data/whois/data.json?resource=${prefix}`,
            success: function (pfx_whois) {
                // let authorities = pfx_whois["data"]["authorities"].map(v => v.toLowerCase());
                // authorities.push("radb");
                let records = pfx_whois["data"]["records"];
                console.log(records);
                if(records.length === 1){
                    whois_dict[prefix] = records;
                    return
                }

                let filtered_records = [];
                records.forEach(function(record){
                    let match = false;
                    record.some(function(elem){
                        if(elem["key"]==="inetnum" || elem["key"]==="CIDR"){
                            match = true;
                            return true
                        }
                    });
                    if(match){
                        filtered_records.push(record);
                    }
                });
                if(filtered_records.length===0){
                    whois_dict[prefix] = records
                } else {
                    whois_dict[prefix] = filtered_records;
                }
            }
        });
    }
}

function load_asrank_content(origins) {
    let origin_lst = origins.split(",");
    origin_lst.forEach(function (origin) {
            $.ajax({
                url: `http://as-rank.caida.org/api/v1/asns/${origin}`,
                success: function (asorg) {
                    if (asorg["data"] != null) {
                        let as_name = process_as_name(asorg["data"]);
                        $(`.as-btn-${origin}`).each(function () {
                            $(this).html(`AS${origin} ${asorg["data"]["country"]} ${as_name}`);
                            $(this).attr("title", `${asorg["data"]["country_name"]}, ${asorg["data"]["org"]["name"]}`)
                        });
                    }
                },
            })
        }
    );

}

function process_as_name(as_org, max_length = 15) {
    if (!("name" in as_org)) {
        return ""
    }

    let as_name = as_org["name"];

    if (as_name.length > max_length - 3) {
        as_name = as_name.toString().substr(0, max_length - 3) + "..."
    }

    console.log(`AS ${as_name}`);
    return as_name
}

function render_origins(origins) {
    let origin_lst = origins.split(",");
    let links = [];

    origin_lst.forEach(function (origin) {
        links.push(`<a class="btn btn-default as-btn as-btn-${origin}" data-toggle="tooltip" title="" data-placement="top" href='http://as-rank.caida.org/asns/${origin}' target="_blank")> AS${origin} </a>`)
    });
    load_asrank_content(origins);

    return links.join(" ")
}

function render_prefix(prefix) {
    let asns = "";
    let prefix_class = prefix.replace("/", "-").replace(/\./g, "-");
    // load_ripe_data(prefix, prefix_class);
    load_who_is(prefix);
    return `<a class="btn btn-default pfx-btn-${prefix_class}" target="_blank" href='https://stat.ripe.net/${prefix}#tabId=at-a-glance')> ${prefix}</a>`
}

function render_traceroutes(data) {
    if (data === undefined || data.length === 0) {
        return "<button disabled> no details </button>"
    } else {
        var uuid = guid();
        traceroute_hash[uuid] = data;
        return `<button onclick='load_traceroute_page("${uuid}")' value=''> details </button>`
    }
}

function load_traceroute_page(uuid) {
    let pfx_event = traceroute_hash[uuid];
    let path = window.location.pathname.replace(/\/$/, "")
    let path_segments = path.split("/");
    let event_type = path_segments[path_segments.length - 2];

    let fingerprint = extract_pfx_event_fingerprint(pfx_event, event_type);
    window.open(`${path}/${fingerprint}`)
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

/* Formatting function for row details - modify as you need */
function format_prefix_table(prefix) {
    // `d` is the original data object for the row
    let thead = '<table cellpadding="5" cellspacing="0" border="1" style="padding-left:50px;">';
    let tfoot = '</table>';
    let tbody = "";

    let records = whois_dict[prefix];
    if(records.length>0){
        records.forEach(function(record){
            record.forEach(function(elem){
                tbody += `<tr><td>${elem["key"]}</td><td>${elem["value"]}</td></tr>`
            });
            tbody += `<tr><td class="bottom-border"></td><td class="bottom-border"></td></tr>`
        });
    } else {
        tbody = "loading information ..."
    }
    return thead+tbody+tfoot;
}

function load_event_details_submoas() {
    $(document).ready(function () {
        var id = window.location.pathname.replace(/\/$/, "").split("/").pop();

        var table = $('#datatable').DataTable({
                "ajax": {
                    "url": "/json/event/id/" + id
                },
                "columns": [
                    {title: "Super Origins", "data": 'super_origins'},
                    {title: "Sub Origins", "data": 'sub_origins'},
                    {title: "Super Prefix", "data": 'super_pfx'},
                    {title: "Sub Prefix", "data": 'sub_pfx'},
                    {title: "Tags", "data": 'tags'},
                    {title: "Traceroutes", "data": 'traceroutes'},
                ],
                "columnDefs": [
                    {
                        "render": function (data, type, row) {
                            return render_origins(data + '');
                        },
                        "targets": [0, 1]
                    },
                    {
                        "render": function (data, type, row) {
                            return render_prefix(data + '');
                        },
                        "targets": [2, 3]
                    },
                    {
                        "render": function (data, type, row) {
                            return render_traceroutes(row)
                        },
                        "targets": [5]
                    }
                ]
            }
        );

        // Add event listener for opening and closing details
        $('#datatable tbody').on('click', 'tr', function () {
            var tr = $(this);
            var row = table.row(tr);

            if (row.child.isShown()) {
                // This row is already open - close it
                row.child.hide();
                tr.removeClass('shown');
            } else {
                // Open this row
                row.child(
                    `<div class="container"> <div class="child"> <h3 class="right">super prefix</h3>` +
                    format_prefix_table(row.data()["super_pfx"])
                    +`</div><div class="child"><h3>sub prefix</h3>`+
                    format_prefix_table(row.data()["sub_pfx"])
                    +`</div></div>`
                ).show();
                tr.addClass('shown');
            }
        });
    })
}

function load_event_details_moas() {
    $(document).ready(function () {
        var id = window.location.pathname.replace(/\/$/, "").split("/").pop();

        var table = $('#datatable').DataTable({
                "ajax": {
                    "url": "/json/event/id/" + id
                },
                "columns": [
                    {title: "Origins", "data": 'origins'},
                    {title: "Newcomer Origins", "data": 'newcomer_origins'}, {title: "Prefix", "data": 'prefix'},
                    {title: "Tags", "data": 'tags'},
                    {title: "Traceroutes", "data": 'traceroutes'},
                ],
                "columnDefs": [
                    {
                        "render": function (data, type, row) {
                            return render_origins(data + '');
                        },
                        "targets": [0, 1]
                    },
                    {
                        "render": function (data, type, row) {
                            return render_prefix(data + '');
                        },
                        "targets": [2]
                    },
                    {
                        "render": function (data, type, row) {
                            return render_traceroutes(row)
                        },
                        "targets": [4]
                    }
                ]
            }
        );

        // Add event listener for opening and closing details
        $('#datatable tbody').on('click', 'tr', function () {
            var tr = $(this);
            var row = table.row(tr);

            if (row.child.isShown()) {
                // This row is already open - close it
                row.child.hide();
                tr.removeClass('shown');
            } else {
                // Open this row
                row.child(
                    `<div class="child"> <h3 class="right">${row.data()["prefix"]} </h3> ${format_prefix_table(row.data()["prefix"])} </div>`
                ).show();
                tr.addClass('shown');
            }
        });
    })
}

function load_event_details_edges() {
    $(document).ready(function () {
        var id = window.location.pathname.replace(/\/$/, "").split("/").pop();

        var table = $('#datatable').DataTable({
                "ajax": {
                    "url": "/json/event/id/" + id
                },
                "columns": [
                    {title: "AS1", "data": 'as1'},
                    {title: "AS2", "data": 'as2'},
                    {title: "Prefix", "data": 'prefix'},
                    {title: "Tags", "data": 'tags'},
                    {title: "Traceroutes", "data": 'traceroutes'},
                ],
                "columnDefs": [
                    {
                        "render": function (data, type, row) {
                            return render_origins(data + '');
                        },
                        "targets": [0, 1]
                    },
                    {
                        "render": function (data, type, row) {
                            return render_prefix(data + '');
                        },
                        "targets": [2]
                    },
                    {
                        "render": function (data, type, row) {
                            return render_traceroutes(row)
                        },
                        "targets": [4]
                    }
                ]
            }
        );

        // Add event listener for opening and closing details
        $('#datatable tbody').on('click', 'tr', function () {
            var tr = $(this);
            var row = table.row(tr);

            if (row.child.isShown()) {
                // This row is already open - close it
                row.child.hide();
                tr.removeClass('shown');
            } else {
                // Open this row
                row.child(
                    `<div class="child"> <h3 class="right">${row.data()["prefix"]} </h3> ${format_prefix_table(row.data()["prefix"])} </div>`
                ).show();
                tr.addClass('shown');
            }
        });
    })
}

function load_event_details_defcon() {
    $(document).ready(function () {
        var id = window.location.pathname.replace(/\/$/, "").split("/").pop();

        var table = $('#datatable').DataTable({
                "ajax": {
                    "url": "/json/event/id/" + id
                },
                "columns": [
                    {title: "Super Prefix", "data": 'super_pfx'},
                    {title: "Sub Prefix", "data": 'sub_pfx'},
                    {title: "Origins", "data": 'origins'},
                    {title: "Tags", "data": 'tags'},
                    {title: "Traceroutes", "data": 'traceroutes'},
                ],
                "columnDefs": [
                    {
                        "render": function (data, type, row) {
                            return render_origins(data + '');
                        },
                        "targets": [2]
                    },
                    {
                        "render": function (data, type, row) {
                            return render_prefix(data + '');
                        },
                        "targets": [0, 1]
                    },
                    {
                        "render": function (data, type, row) {
                            return render_traceroutes(row)
                        },
                        "targets": [4]
                    }
                ]
            }
        );

        // Add event listener for opening and closing details
        $('#datatable tbody').on('click', 'tr', function () {
            var tr = $(this);
            var row = table.row(tr);

            if (row.child.isShown()) {
                // This row is already open - close it
                row.child.hide();
                tr.removeClass('shown');
            } else {
                // Open this row
                row.child(
                    `<div class="container"> <div class="child"> <h3 class="right">${row.data()["super_pfx"]}</h3>
                    ${format_prefix_table(row.data()["super_pfx"])}
                    </div><div class="child"><h3>${row.data()["sub_pfx"]}</h3>
                    ${format_prefix_table(row.data()["sub_pfx"])}
                    </div></div>`
                ).show();
                tr.addClass('shown');
            }
        });
    })
}
