function load_events_table() {
    $(document).ready(function () {

        var table = $('#datatable').DataTable({
                "ajax": {
                    "url": "/json/event/all/100"
                },
                "columns": [
                    {"data": 'event_type'},
                    {"data": 'fingerprint'},
                    {"data": 'id'},
                    {"data": 'pfx_events_cnt'},
                    {"data": 'position'},
                    {"data": 'view_ts'},
                ],
                "columnDefs": [
                    {
                        // The `data` parameter refers to the data for the cell (defined by the
                        // `data` option, which defaults to the column being worked with, in
                        // this case `data: 0`.
                        "render": function (data, type, row) {
                            return "<button>" + data + "</button>";
                        },
                        "targets": 2
                    },
                ]

            }
        );

        $('#datatable tbody').on('click', 'button', function () {

            var data = table.row($(this).parents('tr')).data();

            $.ajax({
                url: "/json/event/id/" + data['id'],
                data: data,
                success: function (data_array) {
                    window.open("event/" + data['event_type'] + "/" + data['id'], "_self");
                }
            });


        });

    })
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

function render_origins(origins) {
    let origin_lst = origins.split(",");
    let links = [];

    origin_lst.forEach(function (origin) {
            links.push('<button class="origin-button" onclick="window.open(\'http:\/\/as-rank.caida.org\/asns\/' + origin + '\')"> ' + origin + ' </button>')
        }
    );
    return links.join(" ")
}

function render_traceroutes(data) {
    if(data === undefined || data.length === 0){
        return "<button disabled> no details </button>"
    } else {
        var uuid = guid();
        traceroute_hash[uuid] = data;
        return `<button onclick='load_traceroute_page("${uuid}")' value=''> details </button>`
    }
}

function load_traceroute_page(uuid){
    let pfx_event = traceroute_hash[uuid];
    let path = window.location.pathname.replace(/\/$/, "")
    let path_segments = path.split("/");
    let event_type = path_segments[path_segments.length -2];

    let fingerprint = extract_pfx_event_fingerprint(pfx_event, event_type);
    window.open(`${path}/${fingerprint}`)
}

function extract_pfx_event_fingerprint(pfx_event, event_type){
    let fingerprint="";
    console.log(pfx_event)
    switch(event_type){
        case "moas":
            fingerprint=`${pfx_event["prefix"]}`;
            break;
        case "submoas":
            fingerprint=`${pfx_event["sub_pfx"]}_${pfx_event["super_pfx"]}`;
            break;
        case "edges":
            fingerprint=`${pfx_event["prefix"]}`;
            break;
        case "defcon":
            fingerprint=`${pfx_event["sub_pfx"]}_${pfx_event["super_pfx"]}`;
            break;
        default:
            alert(`wrong event type ${event_type}`)
    }

    return fingerprint.replace(/\//g, "-")
}

function load_event_details_submoas() {
    $(document).ready(function () {
        var id = window.location.pathname.replace(/\/$/, "").split("/").pop();

        var table = $('#datatable').DataTable({
                "ajax": {
                    "url": "/json/event/id/" + id
                },
                "columns": [
                    {"data": 'super_origins'},
                    {"data": 'sub_origins'},
                    {"data": 'super_pfx'},
                    {"data": 'sub_pfx'},
                    {"data": 'tags'},
                    {"data": 'traceroutes'},
                ],
                "columnDefs": [
                    {
                        "render": function (data, type, row) {
                            return render_origins(data + '');
                        },
                        "targets": [0, 1]
                    },
                    {
                        "render": function(data, type, row){
                            return render_traceroutes(row)
                        },
                        "targets": [5]
                    }
                ]
            }
        );
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
                    {"data": 'origins'},
                    {"data": 'newcomer_origins'},
                    {"data": 'prefix'},
                    {"data": 'tags'},
                    {"data": 'traceroutes'},
                ],
                "columnDefs": [
                    {
                        "render": function (data, type, row) {
                            return render_origins(data + '');
                        },
                        "targets": [0, 1]
                    },
                    {
                        "render": function(data, type, row){
                            return render_traceroutes(row)
                        },
                        "targets": [4]
                    }
                ]
            }
        );
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
                    {"data": 'as1'},
                    {"data": 'as2'},
                    {"data": 'prefix'},
                    {"data": 'tags'},
                    {"data": 'traceroutes'},
                ],
                "columnDefs": [
                    {
                        "render": function (data, type, row) {
                            return render_origins(data + '');
                        },
                        "targets": [0, 1]
                    },
                    {
                        "render": function(data, type, row){
                            return render_traceroutes(row)
                        },
                        "targets": [4]
                    }
                ]
            }
        );
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
                    {"data": 'super_pfx'},
                    {"data": 'sub_pfx'},
                    {"data": 'origins'},
                    {"data": 'tags'},
                    {"data": 'traceroutes'},
                ],
                "columnDefs": [
                    {
                        "render": function (data, type, row) {
                            return render_origins(data + '');
                        },
                        "targets": [2]
                    },
                    {
                        "render": function(data, type, row){
                            return render_traceroutes(row)
                        },
                        "targets": [4]
                    }
                ]
            }
        );
    })
}
