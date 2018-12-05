function create_row(row, row_type, contents) {
    let mapping = {};
    for (let i in contents) {
        let th = document.createElement(row_type);
        th.innerHTML = contents[i];
        row.insertCell().appendChild(th);
        mapping[contents[i]] = i;
    }
    return mapping;
}

function fill_table_row(row, mapping, data) {

    for (let key in mapping) {
        if (key in data) {
            // key is the actual key string shown below
            if (["event_type", "fingerprint", "pfx_events_cnt", "position", "view_ts"].indexOf(key) >= 0) {
                row.insertCell(mapping[key]).appendChild(document.createTextNode(data[key]))
            }

            if (key === "id") {
                let cell = row.insertCell(mapping[key]);

                let a = document.createElement('a');
                a.setAttribute('href', "/event/id/" + data[key]);
                a.innerHTML = data[key];
                cell.appendChild(a);
            }
        }
    }
}

function load_table() {
    $.ajax({
        type: "GET",
        url: '/json/event/all/20',
        success: function (data_array) {
            $("#loading").show()
            let tableRef = document.getElementById("datatable");
            let head = tableRef.createTHead();
            let newRow = head.insertRow();
            let key_mapping = create_row(newRow, 'th', ['event_type', 'fingerprint', 'id', 'pfx_events_cnt', 'position', 'view_ts'])


            let tbody = tableRef.createTBody();
            for (let i in data_array) {
                newRow = tbody.insertRow();
                fill_table_row(newRow, key_mapping, data_array[i]);
            }
            $('#datatable').DataTable();
            $("#loading").hide()
        }
    });
}

function datatable_load() {
    $(document).ready(function () {

        let tableRef = document.getElementById("datatable");
        let head = tableRef.createTHead();
        let newRow = head.insertRow();
        let key_mapping = create_row(newRow, 'th', ['event_type', 'fingerprint', 'id', 'pfx_events_cnt', 'position', 'view_ts'])

        var table = $('#datatable').DataTable({
                "ajax": {
                    "url": "/json/event/all/20"
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
                    alert(JSON.stringify(data_array['pfx_events'][[0]]))
                }
            });


        });

    })
}