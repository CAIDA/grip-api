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
                url: "/json/event/id/"+data['id'],
                data: data,
                success: function(data_array){
                    window.open("event/" + data['event_type'] + "/" + data['id'], "_self");
                }
            });


        });

    })
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
                    {"data": 'tr_worthy'},
                    {"data": 'tags'},
                ],
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
                    {"data": 'tr_worthy'},
                    {"data": 'tags'},
                ],
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
                    {"data": 'tr_worthy'},
                    {"data": 'tags'},
                ],
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
                    {"data": 'tr_worthy'},
                    {"data": 'tags'},
                ],
            }
        );
    })
}
