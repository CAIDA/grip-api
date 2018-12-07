function load_pfx_event() {

    let path = window.location.pathname.replace(/\/$/, "")
    let path_segments = path.split("/");
    let event_id = path_segments[path_segments.length-2];
    let pfx_fingerprint = path_segments[path_segments.length-1];

    $.ajax({
        url: `/json/pfx_event/id/${event_id}/${pfx_fingerprint}`,
        // url: `/json/pfx_event/id/moas-1544142600-12345_57767/pfx`,
        success: function (pfx_event) {
            // window.open("event/" + data['event_type'] + "/" + data['id'], "_self");
            console.log(JSON.stringify(pfx_event));
            $("#json_content").html(syntaxHighlight(JSON.stringify(pfx_event, undefined, 4)));
            console.log(pfx_event)
        }
    });

}
