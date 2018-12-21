/* Formatting function for row details - modify as you need */
function format_prefix_table(prefix) {
    // `d` is the original data object for the row
    let thead = '<table cellpadding="5" cellspacing="0" border="1" style="padding-left:50px;">';
    let tfoot = '</table>';
    let tbody = "";

    let records = whois_dict[prefix];
    if(records.length>0){
        records.forEach(function(record){
            record.forEach(function(elem){
                tbody += `<tr><td>${elem["key"]}</td><td>${elem["value"]}</td></tr>`
            });
            tbody += `<tr><td class="bottom-border"></td><td class="bottom-border"></td></tr>`
        });
    } else {
        tbody = "loading information ..."
    }
    return thead+tbody+tfoot;
}

function render_origin_links(origins) {
    let origin_lst = origins.split(",");
    let links = [];

    origin_lst.forEach(function (origin) {
        links.push(`<a class="btn btn-default as-btn as-btn-${origin}" data-toggle="tooltip" title="" data-placement="top" href='http://as-rank.caida.org/asns/${origin}' target="_blank")> AS${origin} </a>`)
    });
    load_origins_asrank(origins);

    return links.join(" ")
}

function render_prefix_link(prefix) {
    let asns = "";
    let prefix_class = prefix.replace("/", "-").replace(/\./g, "-");
    // load_ripe_data(prefix, prefix_class);
    load_who_is(prefix);
    return `<a class="btn btn-default pfx-btn-${prefix_class}" target="_blank" href='https://stat.ripe.net/${prefix}#tabId=at-a-glance')> ${prefix}</a>`
}

function render_traceroutes_link(data) {
    if (data === undefined || data.length === 0) {
        return "<button disabled> no details </button>"
    } else {
        var uuid = get_guid();
        traceroute_hash[uuid] = data;
        return `<button onclick='load_traceroute_page("${uuid}")' value=''> details </button>`
    }
}

