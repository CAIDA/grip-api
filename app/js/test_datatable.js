table_data=` <thead>
                  <tr>
                    <th>Name</th>
                    <th>Position, yaaa</th>
                    <th>Office</th>
                    <th>Age</th>
                    <th>Start date</th>
                    <th>Salary</th>
                  </tr>
                </thead>


                <tbody>
                  <tr>
                    <td>Donna Snider</td>
                    <td>Customer Support</td>
                    <td>New York</td>
                    <td>27</td>
                    <td>2011/01/25</td>
                    <td>$112,000</td>
                  </tr>
                </tbody>
                `
function create_row(row, row_type, contents){
    var mapping = {}
    for(var i in contents){
        var th = document.createElement(row_type);
        th.innerHTML=contents[i];
        row.insertCell().appendChild(th);
        mapping[contents[i]] = i;
    }
    return mapping;
}

function fill_table_row(row, mapping, data){
    for(var key in mapping){
        if(key in data){
            if (["event_type", "fingerprint", "id", "pfx_events_cnt", "position", "view_ts"].indexOf(key)>=0) {
                row.insertCell(mapping[key]).appendChild(document.createTextNode(data[key]))
            }
            // if (key == "pfx_events") {
            //     console.log("pfx_events "+ mapping[key]);
            //     var a = document.createElement('a');
            //     a.appendChild(document.createTextNode("here is a link to pfx events"));
            //     row.insertCell(mapping[key]).appendChild(a)
            // }
            // if (key == "tags") {
            //     console.log(data[key].join());
            //     row.insertCell(mapping[key]).appendChild(document.createTextNode(data[key].join()))
            // }
        }
    }
}

function load_table() {
    $.ajax({
        type: "GET",
        url: '/example',
        success: function (data) {
            var tableRef = document.getElementById("datatable");
            var head = tableRef.createTHead();
            var newRow = head.insertRow();
            // var key_mapping = create_row(newRow, 'th', ['event_type','fingerprint','id','pfx_events','pfx_events_cnt','position','tags','tr_metrics','view_ts'])
            var key_mapping = create_row(newRow, 'th', ['event_type','fingerprint','id','pfx_events_cnt','position','view_ts'])

            var tbody = tableRef.createTBody();
            newRow = tbody.insertRow();
            fill_table_row(newRow, key_mapping, data);
        }

    });
}