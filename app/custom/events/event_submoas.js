function get_submoas_columns() {
    return [
        {"title": "Super Origins", "data": 'super_origins'},
        {"title": "Sub Origins", "data": 'sub_origins'},
        {"title": "Super Prefix", "data": 'super_pfx'},
        {"title": "Sub Prefix", "data": 'sub_pfx'},
        {"title": "Tags", "data": 'tags'},
        {"title": "Traceroutes", "data": 'traceroutes'},
    ];
}

function get_submoas_column_defs() {
    return [
        {
            "render": function (data, type, row) {
                return render_origin_links(data + '');
            },
            "targets": [0, 1]
        },
        {
            "render": function (data, type, row) {
                return render_prefix_link(data + '');
            },
            "targets": [2, 3]
        },
        {
            "render": function (data, type, row) {
                return render_traceroutes_link(row)
            },
            "targets": [5]
        }

    ];
}

function submoas_prefix_details(table){
    // Add event listener for opening and closing details
    $('#datatable tbody').on('click', 'tr', function () {
        let tr = $(this);
        let row = table.row(tr);

        if (row.child.isShown()) {
            // This row is already open - close it
            row.child.hide();
            tr.removeClass('shown');
        } else {
            // Open this row
            row.child(
                `<div class="container"> <div class="child"> <h3 class="right">super prefix</h3>` +
                format_prefix_table(row.data()["super_pfx"])
                + `</div><div class="child"><h3>sub prefix</h3>` +
                format_prefix_table(row.data()["sub_pfx"])
                + `</div></div>`
            ).show();
            tr.addClass('shown');
        }
    });
}
