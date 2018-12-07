function load_pfx_event() {

    $.ajax({
        url: "/json/pfx_event/id/moas-1544142600-12345_57767/pfx",
        success: function (data_array) {
            // window.open("event/" + data['event_type'] + "/" + data['id'], "_self");
            alert(JSON.stringify(data_array));
            console.log(JSON.stringify(data_array))
        }
    });

}
