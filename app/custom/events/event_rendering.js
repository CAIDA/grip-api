let table_info_dict = {};

function load_event_scripts() {
    let script_paths = [
        "/app/custom/events/event_submoas.js",
        "/app/custom/events/event_moas.js",
        "/app/custom/events/event_defcon.js",
        "/app/custom/events/event_edges.js",
    ];

    for (let i in script_paths) {
        $.ajax({
            url: script_paths[i],
            dataType: "script",
            async: false,
        });
    }

    table_info_dict = {
        "submoas": {
            "columns": get_submoas_columns(),
            "columnDefs": get_submoas_column_defs(),
            "pfx_details_func": submoas_prefix_details,
        },
        "edges": {
            "columns": get_edges_columns(),
            "columnDefs": get_edges_column_defs(),
            "pfx_details_func": edges_prefix_details,
        },
        "defcon": {
            "columns": get_defcon_columns(),
            "columnDefs": get_defcon_column_defs(),
            "pfx_details_func": defcon_prefix_details,
        },
        "moas": {
            "columns": get_moas_columns(),
            "columnDefs": get_moas_column_defs(),
            "pfx_details_func": moas_prefix_details,
        },
    };
}


function render_pfx_event_table(event_type, event, table_id = "#datatable", paging = true) {

    if (isEmpty(table_info_dict)) {
        load_event_scripts()
    }

    console.log("event:!");
    console.log(event);

    // render table based on event types
    let table = $(table_id).DataTable({
        data: event,
        paging: paging,
        searching: false,
        "columns": table_info_dict[event_type]["columns"],
        "columnDefs": table_info_dict[event_type]["columnDefs"],
    });

    $('#datatable tbody').on('click', 'tr', function () {
        var data = table.row($(this)).data();
        let path = window.location.pathname.replace(/\/$/, "");
        let fingerprint = extract_pfx_event_fingerprint(data, event_type);
        window.open(`${path}/${fingerprint}`, "_self", false)
    });
}

function render_impact(num_pfx, num_addrs){
    let impact_str = "";
    if(num_pfx === 1){
        impact_str+= `${num_pfx} pfx `
    } else {
        impact_str+= `${num_pfx} pfxs `
    }
    if(num_addrs === 1){
        impact_str+= `(${num_addrs} addr)`
    } else {
        impact_str+= `(${num_addrs} addrs)`
    }
    return impact_str
}


function render_event_details_table(event_type, event){
    console.log(event);
    $("#event-details-victim").html(
        render_origin_links(
            extract_victims(event["pfx_events"][0], event_type,), 2
        )
    );
    $("#event-details-attacker").html(
        render_origin_links(
            extract_attackers(event["pfx_events"][0], event_type), 2
        )
    );
    $("#event-details-prefix").text(extract_largest_prefix(event["pfx_events"]));
    let [num_pfx, num_addrs] = extract_impact(event["pfx_events"]);
    $("#event-details-impact").text(render_impact(num_pfx,num_addrs));
    $("#event-details-startts").text(event["view_ts"]);
    $("#event-details-type").text(event_type_explain[event_type]);

    if (event["finished_ts"] === null) {
        $("#event-details-duration").text("On-Going");
        $("#event-details-endts").text("N/A");
    } else {
        start_ts = Date.parse(event["view_ts"]);
        end_ts = Date.parse(event["finished_ts"]);
        $("#event-details-duration").text(`${(end_ts-start_ts)/1000/60} min`);
        $("#event-details-endts").text(event["finished_ts"]);
    }
}

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

function render_origin_links(origin_lst, style = 1) {
    let links = [];
    if(origin_lst[0] === ""){
        return "N/A"
    }

    origin_lst.forEach(function (origin) {
        // links.push(`<a class="btn btn-default as-btn as-btn-${origin}" data-toggle="tooltip" title="" data-placement="top" href='http://as-rank.caida.org/asns/${origin}' target="_blank")> AS${origin} </a>`)
        links.push(`<a class="link as-btn as-btn-${origin}" data-toggle="tooltip" title="" data-placement="top" href='http://as-rank.caida.org/asns/${origin}' target="_blank")> AS${origin} </a>`)
    });
    load_origins_asrank(origin_lst, style);

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

function capitalizeFirstLetter(string) {
    return string.charAt(0).toUpperCase() + string.slice(1);
}

worthy_tag_dict = {
    "moas": {
        "yes": [
            "moas-short-prefix",
            "moas-not-previously-announced-by-any-newcomer",
        ],
        "no": [
            "moas-recurring-pfx-event",
            "moas-not-previously-announced",
            "moas-reserved-space",
            "moas-no-newcomer",
            "moas-all-newcomers",
            "moas-no-prev-origin",
            "moas-due-to-private-asn",
            "moas-all-newcomers-private-asn",
            "moas-due-to-dps-asn",
            "moas-all-newcomers-dps-asn",
            "moas-previously-announced-by-all-newcomers",
            "moas-ixp-prefix",
            "moas-siblings",
            "moas-pc-single-upstream",
            "moas-newcomer-is-direct-provider",
            "moas-newcomer-is-direct-customer",
            "moas-newcomer-is-direct-peer",
            "moas-oldasn-in-newcomer-customer-cone",
            "moas-newcomer-in-oldasn-customer-cone",
            "moas-multi-all-siblings",
            "moas-multi-pc-single-upstream-chain",
            "moas-multi-newcomers-are-all-direct-providers",
            "moas-multi-newcomers-are-all-direct-customers",
            "moas-multi-newcomers-are-all-direct-peers",
            "moas-multi-newcomers-are-all-upstreams",
            "moas-multi-newcomers-are-all-downstreams",
        ],
        "na": [
            "moas-notags",
            "moas-outdated-info",
            "moas-outdated-maybe-not-no-newcomer",
            "moas-outdated-maybe-not-all-newcomers",
            "moas-outdated-maybe-not-no-prev-origin",
            "moas-has-private-asn",
            "moas-has-private-newcomer-asn",
            "moas-has-dps-asn",
            "moas-has-dps-newcomer-asn",
            "moas-previously-announced-by-newcomer",
            "moas-newcomer-is-directly-connected",
            "moas-multi-some-siblings",
            "moas-multi-newcomers-are-all-directly-connected",
            "moas-multi-some-newcomers-are-direct-providers",
            "moas-multi-some-newcomers-are-direct-customers",
            "moas-multi-some-newcomers-are-direct-peers",
            "moas-multi-some-newcomers-are-directly-connected",
            "moas-multi-some-newcomers-are-upstreams",
            "moas-multi-some-newcomers-are-downstreams",
        ],
    },
    "submoas": {
        "yes": [
            "submoas-short-prefix-super_prefix",
            "submoas-short-prefix",
            "submoas-not-previously-announced-by-any-newcomer"
        ],
        "no": [
            "submoas-recurring-pfx-event",
            "submoas-not-previously-announced",
            "submoas-no-newcomer",
            "submoas-no-newcomer-pfxs",
            "submoas-all-newcomers",
            "submoas-all-newcomers-pfxs",
            "submoas-due-to-private-asn",
            "submoas-all-newcomers-private-asn",
            "submoas-due-to-dps-asn",
            "submoas-all-newcomers-dps-asn",
            "submoas-previously-announced-by-all-newcomers",
            "submoas-ixp-prefix",
            "submoas-siblings",
            "submoas-pc-single-upstream",
            "submoas-newcomer-is-direct-provider",
            "submoas-newcomer-is-direct-customer",
            "submoas-newcomer-is-direct-peer",
            "submoas-newcomer-is-directly-connected",
            "submoas-oldasn-in-newcomer-customer-cone",
            "submoas-newcomer-in-oldasn-customer-cone",
            "submoas-multi-all-siblings",
            "submoas-multi-pc-single-upstream-chain",
            "submoas-multi-newcomers-are-all-direct-providers",
            "submoas-multi-newcomers-are-all-direct-customers",
            "submoas-multi-newcomers-are-all-direct-peers",
            "submoas-multi-newcomers-are-all-directly-connected",
            "submoas-multi-newcomers-are-all-upstreams",
            "submoas-multi-newcomers-are-all-downstreams",
        ],
        "na":[
            "submoas-notags",
            "submoas-trans-asn",
            "submoas-has-private-asn",
            "submoas-has-private-newcomer-asn",
            "submoas-has-dps-asn",
            "submoas-has-dps-newcomer-asn",
            "submoas-newcomer-more-specific",
            "submoas-previously-announced-by-some-newcomers",
            "submoas-multi-some-siblings",
            "submoas-multi-some-newcomers-are-direct-providers",
            "submoas-multi-some-newcomers-are-direct-customers",
            "submoas-multi-some-newcomers-are-direct-peers",
            "submoas-multi-some-newcomers-are-directly-connected",
            "submoas-multi-some-newcomers-are-upstreams",
            "submoas-multi-some-newcomers-are-downstreams",
        ],
    },
    "defcon": {
        "yes": [
            "defcon-prefixes-reserved-space",
            "defcon-sub-path-longer",
        ],
        "no": [
            "defcon-recurring-pfx-event",
            "defcon-prefixes-newcomer-super-prefix",
            "defcon-sub-path-shorter",
            "defcon-superpaths-include-subpaths",
            "defcon-no-common-monitors",
        ],
        "na": [
            "defcon-sub-super-equal",
            "defcon-prefixes-sub-pfx-no-common-hops",
            "defcon-prefixes-super-pfx-no-common-hops",
            "defcon-prefixes-all-pfx-no-common-hops",
        ],
    },
    "edges": {
        "yes": [
            "edge-reserved-asn",
            "edge-short-prefix",
            "edge-reserved-space",
            "edge-unallocated-asn"
        ],
        "no": [
            "edge-recurring-pfx-event",
            "edge-new-bidirectional",
            "edge-ixp-colocated",
            "edge-siblings",
            "edge-dps-asn",
            "edge-adj-previously-observed-exact",
            "edge-adj-previously-observed",
            "edge-private-asn",
        ],
        "na": [
            "edge-not-in-customer-cone",
            "edge-notags",
            "edge-assigned-asn",
            "edge-trans-asn"
        ],
    }
};

tag_type = {};
tag_type_ready = false;

type_label = {
    "yes": "label-danger",
    "no": "label-success",
    "na": "label-default",
};

function update_tag_type(){
    if(tag_type_ready){
        return
    }
    for(let type in worthy_tag_dict){
        for(let nature in worthy_tag_dict[type]){
            for(let index in worthy_tag_dict[type][nature]){
                let tag = worthy_tag_dict[type][nature][index]
                tag_type[tag] = nature
            }
        }
    }
}

function render_tag_name(tag){
    return tag.split("-")
        .map(
            function(x){
                return capitalizeFirstLetter(x)
            })
        .slice(1)
        .join(" ")
}

function render_tags(tags){
    update_tag_type();
    entries = [];
    for(let i in tags){
        let tag = tags[i];
        if(!(tag in tag_type)){
            entries.push(`<span style="color: purple; ">${tag}</span>`)
        }
        entries.push(`<span class="label ${type_label[tag_type[tag]]}" style="font-size: 12px;" data-toggle='tooltip' title='${tag_type[tag]}'>${render_tag_name(tag)}</span></h4>`)
    }
    return entries.join(" &nbsp; ")
}
