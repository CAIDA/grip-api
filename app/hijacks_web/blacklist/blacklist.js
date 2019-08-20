function load_blacklist(){
    let blacklist = [];
    $.ajax({
        dataType: "json",
        async: false,
        url: "/json/blacklist",
        success: function (data) {
            for(let asn of data['blacklist']){
                blacklist.push([asn])
            }
        }
    });
    $('#datatable').DataTable({
        searching: false,
        ordering: false,
        paging: false,
        data:blacklist,
        columns: [
            {title: "Blacklist AS"},
        ],
        columnDefs: [
            {
                "render": function (data, type, row) {
                    return render_origin_links( [data], true);
                },
                "targets": [0]
            }
        ]
    })
}

