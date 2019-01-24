function get_edges_columns() {
    return [
        {"title": "Prefix", "data": 'prefix'},
        {"title": "Tags", "data": 'tags'},
        {"title": "Traceroutes", "data": 'traceroutes'},
    ];
}

function get_edges_column_defs() {
    return [
        {
            "render": function (data, type, row) {
                if(data.length > 0){
                    return "yes"
                } else {
                    return "no"
                }
                // return render_traceroutes_link(row)
            },
            "targets": [2]
        }
    ];
}

function edges_prefix_details(table){
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
}
