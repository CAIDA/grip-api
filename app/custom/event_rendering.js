let table_info_dict = {};

function load_event_scripts() {
    let script_paths = [
        "/app/custom/events/event_submoas.js",
        "/app/custom/events/event_moas.js",
        "/app/custom/events/event_defcon.js",
        "/app/custom/events/event_edges.js",
        "/app/custom/events/common.js",
    ];

    for (let i in script_paths) {
        $.ajax({
            url: script_paths[i],
            dataType: "script",
            async: false,
        });
    }

    table_info_dict = {
        "submoas": {
            "columns": get_submoas_columns(),
            "columnDefs": get_submoas_column_defs(),
            "pfx_details_func": submoas_prefix_details,
        },
        "edges": {
            "columns": get_edges_columns(),
            "columnDefs": get_edges_column_defs(),
            "pfx_details_func": edges_prefix_details,
        },
        "defcon": {
            "columns": get_defcon_columns(),
            "columnDefs": get_defcon_column_defs(),
            "pfx_details_func": defcon_prefix_details,
        },
        "moas": {
            "columns": get_moas_columns(),
            "columnDefs": get_moas_column_defs(),
            "pfx_details_func": moas_prefix_details,
        },
    };
}


function render_pfx_event_table(event_type, event, table_id = "#datatable", paging = true) {

    if (isEmpty(table_info_dict)) {
        load_event_scripts()
    }

    console.log("event:!");
    console.log(event);

    // render table based on event types
    let table = $(table_id).DataTable({
        data: event,
        paging: paging,
        searching: false,
        "columns": table_info_dict[event_type]["columns"],
        "columnDefs": table_info_dict[event_type]["columnDefs"],
    });

    // render pfx details
    table_info_dict[event_type]["pfx_details_func"](table);
}
