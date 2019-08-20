function load_tags(){
    let tags = [];
    let tr = [];
    $.ajax({
        dataType: "json",
        async: false,
        url: "/json/tags",
        success: function (data) {
            for(let tag in data['definitions']){
                let d =  data['definitions'][tag];
                tags.push([tag, d["definition"]])
            }

            for(let entry of data['tr_worthy']){
                tr.push([entry['tags'], entry['worthy'], entry["explain"], entry["apply_to"]])
            }
        }
    });

    $('#tags').DataTable({
        searching: false,
        ordering: false,
        paging: false,
        data:tags,
        columns: [
            {title: "Tag ID"},
            {title: "Definition"},
        ],
        columnDefs: [
        ]
    });

    $('#tr_worthy').DataTable({
        searching: false,
        ordering: false,
        paging: false,
        data:tr,
        columns: [
            {title: "Tags"},
            {title: "TR Worthy"},
            {title: "Explain"},
            {title: "Apply To"},
        ],
        columnDefs: [
        ]
    })
}
