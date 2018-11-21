function create_row(row, row_type, contents){
    let mapping = {};
    for(let i in contents){
        let th = document.createElement(row_type);
        th.innerHTML=contents[i];
        row.insertCell().appendChild(th);
        mapping[contents[i]] = i;
    }
    return mapping;
}

function fill_table_row(row, mapping, data){
    // TODO: add link to event id to call API and return the raw JSON object
    // TODO: add link to pfx_event number to link to the pfx_event list page

    for(let key in mapping){
        if(key in data){
            // key is the actual key string shown below
            if (["event_type", "fingerprint", "pfx_events_cnt", "position", "view_ts"].indexOf(key)>=0) {
                row.insertCell(mapping[key]).appendChild(document.createTextNode(data[key]))
            }

            if (key === "id") {
                let cell = row.insertCell(mapping[key]);

                let a = document.createElement('a');
                a.setAttribute('href',"/json/"+data[key]);
                a.innerHTML = data[key];
                cell.appendChild(a);
            }
        }
    }
}

function load_table() {
    $.ajax({
        type: "GET",
        url: '/example',
        success: function (data_array) {
            let tableRef = document.getElementById("datatable");
            let head = tableRef.createTHead();
            let newRow = head.insertRow();
            let key_mapping = create_row(newRow, 'th', ['event_type','fingerprint','id','pfx_events_cnt','position','view_ts'])

            let tbody = tableRef.createTBody();
            for(let i in data_array){
                newRow = tbody.insertRow();
                fill_table_row(newRow, key_mapping, data_array[i]);
            }
            $('#datatable').DataTable();
        }
    });
}