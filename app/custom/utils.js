event_type_explain = {
    'moas': "origin hijack (moas)",
    'submoas': "origin hijack (submoas)",
    'edges': "path manipulation (new edge)",
    'defcon': "path manipulation (defcon)",
};

function abbrFit(string, nChars, divPos, sep) {
        // The relative position where to place the '...'
        divPos = divPos || 0.7;
        sep = sep || '...';
        if (nChars<=sep.length) sep=''; // If string is smaller than separator

        nChars-=sep.length;

        if (string.length<=nChars) return ""+string;

        return string.substring(0,nChars*divPos)
            + sep
            + string.substring(string.length - nChars*(1-divPos), string.length);
}

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
    const elems = window.location.pathname.replace(/\/$/, "").split("/");
    return elems[3]

}

function get_event_type_from_url(){
    const elems =  window.location.pathname.replace(/\/$/, "").split("/");
    return elems[2]
}

function get_guid() {
    function s4() {
        return Math.floor((1 + Math.random()) * 0x10000)
            .toString(16)
            .substring(1);
    }

    return s4() + s4() + '-' + s4() + '-' + s4() + '-' + s4() + '-' + s4() + s4() + s4();
}

function process_as_name(as_org, max_length = 25) {
    if (!("name" in as_org)) {
        return "Null"
    }
    let as_name = as_org["name"];
    as_name = abbrFit(as_name, max_length - 3);
    return as_name
}

function isEmpty(obj) {
  return Object.keys(obj).length === 0;
}

function extract_victims(pfxevent, event_type){
    switch(event_type){
        case "moas":
            let oldcomers = new Set();
            for(let i in pfxevent["origins"]){
                oldcomers.add(pfxevent['origins'][i]);
            }
            for(let i in pfxevent["newcomer_origins"]){
                oldcomers.delete(pfxevent['newcomer_origins'][i])
            }
            return [...oldcomers];
        case "submoas":
            return pfxevent["sub_origins"];
        case "defcon":
            return pfxevent["origins"];
        case "edges":
            return [pfxevent["as1"], pfxevent["as2"]];
        default:
            return ["wrong"]
    }
}

function extract_attackers(pfxevent, event_type){
    switch(event_type){
        case "moas":
            return pfxevent["newcomer_origins"];
        case "submoas":
            return pfxevent["super_origins"];
        case "defcon":
            return [""];
        case "edges":
            // return [pfxevent["as1"], pfxevent["as2"]];
            return [""];
        default:
            return ["wrong"]
    }
}

function extract_largest_prefix(pfx_events){
    let largest_pfx_len = 1000;
    let largest_pfx = "";
    for(let i in pfx_events){
        let pfxevent = pfx_events[i];
        let p = "";
        if("prefix" in pfxevent){
            p = pfxevent["prefix"];
        } else {
            p = pfxevent["sub_pfx"];
        }
        let len = parseInt(p.split("/")[1]);
        if(len <= largest_pfx_len){
            largest_pfx = p;
            largest_pfx_len = len;
        }
    }
    return largest_pfx;
}

function extract_impact(pfx_events){
    let num_pfx = 0;
    let num_addrs = 0;
    for(let i in pfx_events){
        num_pfx++;
        let pfxevent = pfx_events[i];
        let p = "";
        if("prefix" in pfxevent){
            p = pfxevent["prefix"];
        } else {
            p = pfxevent["sub_pfx"];
        }
        let len = parseInt(p.split("/")[1]);
        if(len<=32){
            num_addrs += Math.pow(2, 32-len);
        } else {
            num_addrs += Math.pow(2, 128-len);
        }
    }
    if(num_addrs.toString().length>10){
        num_addrs = num_addrs.toPrecision(2)
    }

    return [num_pfx, num_addrs]
}

