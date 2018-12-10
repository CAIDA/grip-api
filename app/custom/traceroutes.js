function load_pfx_event() {

    let path = window.location.pathname.replace(/\/$/, "");
    let path_segments = path.split("/");
    let event_id = path_segments[path_segments.length-2];
    let pfx_fingerprint = path_segments[path_segments.length-1];

    $.ajax({
        url: `/json/pfx_event/id/${event_id}/${pfx_fingerprint}`,
        // url: `/json/pfx_event/id/moas-1544142600-12345_57767/pfx`,
        success: function (pfx_event) {
            // window.open("event/" + data['event_type'] + "/" + data['id'], "_self");
            $("#json_content").html(syntaxHighlight(JSON.stringify(pfx_event, undefined, 4)));
            draw_sankey(pfx_event);
        }
    });
}

function extract_sankey_data(path_lst){
    path_count_dict = {};

    path_lst.forEach(function(path){
        path = path.trim().replace(/ {2}/g, ' ');
        let asns = path.split(" ");
        if(asns.length>1){
            for(let i=0;i<asns.length-1; i++){
                if(asns[i] === asns[i+1]){
                    continue
                }
                let segment = `${asns[i]},${asns[i+1]}`;
                if(asns[i] === " " || asns[i+1] === " "){
                    alert(`${path} => ${asns} => ${i}: "${asns[i]}" "${asns[i+1]}"`)
                }
                if(!(segment in path_count_dict)){
                    path_count_dict[segment] = 0
                }
                path_count_dict[segment] ++
            }
        }
    });

    let data = [];
    for(let key in path_count_dict){
        let nodes = key.split(",");
        data.push([nodes[0], nodes[1], path_count_dict[key]])
    }

    console.log(data);
    return data
}

function draw_sankey(pfx_event) {
    google.charts.load('current', {'packages': ['sankey']});
    google.charts.setOnLoadCallback(drawChart);

    let path_data = [];
    if("aspaths" in pfx_event) {
        path_data = extract_sankey_data(pfx_event["aspaths"])
    } else if ("super_aspaths" in pfx_event) {
        path_data = extract_sankey_data(pfx_event["super_aspaths"])
    } else {
        alert("no paths data available")
    }

    function drawChart() {
        var data = new google.visualization.DataTable();
        data.addColumn('string', 'From');
        data.addColumn('string', 'To');
        data.addColumn('number', 'Weight');
        data.addRows(path_data);

        // Sets chart options.
        var options = {
            width: 1200,
            height: data.getNumberOfRows() * 11 + 30
        };

        // Instantiates and draws our chart, passing in some options.
        var chart = new google.visualization.Sankey(document.getElementById('sankey_diagram'));
        chart.draw(data, options);
    }
}

function draw_traceroute(pfx_event) {

}