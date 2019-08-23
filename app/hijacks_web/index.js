let cidr_loose_re = /^[0-9]+[.:][0-9.:/]*$/;
const params = new Map(location.search.slice(1).split("&").map(kv => kv.split("=")));
let event_type_explain = {
    'moas': "origin hijack (moas)",
    'submoas': "origin hijack (submoas)",
    'edges': "path manipulation (new edge)",
    'defcon': "path manipulation (defcon)",
};

// decimal offset between ASCII capitals and regional indicator symbols
const OFFSET = 127397;
function load_scripts() {
    let script_paths = [
        "/app/hijacks_web/common.js",
        "/app/hijacks_web/pfx_event_details/pfx_event_details.js",
        "/app/hijacks_web/event_details/event_details.js",
        "/app/hijacks_web/event_list/event_list.js",
        "/app/hijacks_web/blacklist/blacklist.js",
        "/app/hijacks_web/tags/tags.js",
        "/app/hijacks_web/external_data.js",
    ];
// <script src="https://stat.ripe.net/widgets/widget_api.js"></script>

    for (let i in script_paths) {
        $.ajax({
            url: script_paths[i],
            dataType: "script",
            async: false,
        });
    }
}

load_scripts();