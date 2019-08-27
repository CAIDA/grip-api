
// module exposes a single function
function flag(country_code) {
    // only allow string input
    if (typeof country_code !== 'string'){
        // throw new TypeError('argument must be a string');
        return "";
    }
    // ensure country code is all caps
    const cc = country_code.toUpperCase();
    // return the emoji flag corresponding to country_code or null
    return (/^[A-Z]{2}$/.test(cc))
        ? String.fromCodePoint(...[...cc].map(c => c.charCodeAt() + OFFSET))
        : null;
}

function flag_set(flag_name, params){
    let found = false;
    params.forEach(function(value, key){
        if(key === flag_name && value === "true"){
            console.log(flag_name, "found");
            found = true
        }
    });
    return found;
}

function abbrFit(string, nChars, divPos, sep) {
    // The relative position where to place the '...'
    divPos = divPos || 0.7;
    sep = sep || '...';
    if (nChars<=sep.length) {
        // If string is smaller than separator
        sep='';
    }

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

function extract_largest_prefix(prefixes){
    let largest_pfx_len = 1000;
    let largest_pfx = "";
    for(let p of prefixes){
        let len = parseInt(p.split("/")[1]);
        if(len <= largest_pfx_len){
            largest_pfx = p;
            largest_pfx_len = len;
        }
    }
    return largest_pfx;
}

function extract_impact(prefixes){
    let num_pfx = 0;
    let num_addrs = 0;
    for(let p of prefixes){
        num_pfx++;
        // if("prefix" in pfxevent){
        //     p = pfxevent["prefix"];
        // } else {
        //     p = pfxevent["sub_pfx"];
        // }
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

function extract_pfx_event_fingerprint(pfx_event, event_type) {
    let fingerprint = "";
    switch (event_type) {
        case "moas":
            fingerprint = `${pfx_event["prefix"]}`;
            break;
        case "submoas":
            fingerprint = `${pfx_event["sub_pfx"]}_${pfx_event["super_pfx"]}`;
            break;
        case "edges":
            fingerprint = `${pfx_event["prefix"]}`;
            break;
        case "defcon":
            fingerprint = `${pfx_event["sub_pfx"]}_${pfx_event["super_pfx"]}`;
            break;
        default:
            alert(`wrong event type ${event_type}`)
    }

    return fingerprint.replace(/\//g, "-")
}
