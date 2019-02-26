function get_moas_columns() {
    return [
        {"title": "Prefix", "data": 'prefix'},
        {"title": "Tags", "data": 'tags'},
        {"title": "Traceroutes Available", "data": 'traceroutes'},
    ];
}

function get_moas_column_defs() {
    return [
        {
            "render": function (data, type, row) {
                return render_prefix_link(data)
            },
            "targets": [0]
        },
        {
            "render": function (data, type, row) {
                return render_tags(data)
            },
            "targets": [1]
        },
        {
            "render": function (data, type, row) {
                if(data.length > 0){
                    return "yes"
                } else {
                    return "no"
                }
            },
            "targets": [2]
        }
    ];
}

function moas_prefix_details(table){
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
