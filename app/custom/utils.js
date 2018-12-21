// reference: https://stackoverflow.com/a/7220510/768793
function syntaxHighlight(json) {
    json = json.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    return json.replace(/("(\\u[a-zA-Z0-9]{4}|\\[^u]|[^\\"])*"(\s*:)?|\b(true|false|null)\b|-?\d+(?:\.\d*)?(?:[eE][+\-]?\d+)?)/g, function (match) {
        var cls = 'number';
        if (/^"/.test(match)) {
            if (/:$/.test(match)) {
                cls = 'key';
            } else {
                cls = 'string';
            }
        } else if (/true|false/.test(match)) {
            cls = 'boolean';
        } else if (/null/.test(match)) {
            cls = 'null';
        }
        return '<span class="' + cls + '">' + match + '</span>';
    });
}
function get_event_id_from_url(){
  return window.location.pathname.replace(/\/$/, "").split("/").pop();
}

function get_event_type_from_url(){
    const elems =  window.location.pathname.replace(/\/$/, "").split("/");
    return elems[elems.length-2]
}

function get_guid() {
    function s4() {
        return Math.floor((1 + Math.random()) * 0x10000)
            .toString(16)
            .substring(1);
    }

    return s4() + s4() + '-' + s4() + '-' + s4() + '-' + s4() + '-' + s4() + s4() + s4();
}

function process_as_name(as_org, max_length = 15) {
    if (!("name" in as_org)) {
        return ""
    }

    let as_name = as_org["name"];

    if (as_name.length > max_length - 3) {
        as_name = as_name.toString().substr(0, max_length - 3) + "..."
    }

    console.log(`AS ${as_name}`);
    return as_name
}

