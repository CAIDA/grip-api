function get_defcon_columns() {
    return [
        {"title": "Sub Prefix", "data": 'sub_pfx'},
        {"title": "Super Prefix", "data": 'super_pfx'},
        {"title": "Tags", "data": 'tags'},
        {"title": "Traceroutes Worthy", "data": 'tr_worthy'},
        {"title": "Traceroutes Available", "data": 'traceroutes'},
    ];
}

function get_defcon_column_defs() {
    return [
        {
            "render": function (data, type, row) {
                return render_prefix_link(data)
            },
            "targets": [0,1]
        },
        {
            "render": function (data, type, row) {
                return render_tags("defcon", data)
            },
            "targets": [2]
        },
        {
            "render": function (data, type, row) {
                if(data){
                    return "yes"
                } else {
                    return "no"
                }
            },
            "width": "140px",
            "targets": [3]
        },
        {
            "render": function (data, type, row) {
                return render_tr_availability(data, row)
            },
            "width": "140px",
            "targets": [4]
        }
    ];
}

function defcon_prefix_details(table){
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