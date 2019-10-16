let table_info_dict = {};
let tags_info_dict = {};
let traceroute_hash = {};


function isEmpty(obj) {
    return Object.keys(obj).length === 0;
}

event_modal_info = {
    "download_path": "",
    "json_raw_str": "",
    "content_id": "#json_modal_event",
    "button_class": ".full-event-modal-download",
    "anchorId": 'downloadAnchorElemEvent'
};

function load_event_details() {
    $(document).ready(function () {
        $('body').tooltip({selector: '[data-toggle="tooltip"]'});
        const event_id = get_event_id_from_url();
        const event_type = get_event_type_from_url();
        load_event_scripts();

        $.ajax({
            url: "/json/event/id/" + event_id,
            success: function (event) {
                // console.log(event);
                console.log(event);
                render_event_details_table(event_type, event);
                render_pfx_event_table(event_type, event['pfx_events'], event['tr_metrics']['tr_skipped'], event['tr_metrics']['tr_skip_reason']);
            }
        });
    })
}


function load_event_scripts() {
    let script_paths = [
        "/app/hijacks_web/event_details/event_submoas.js",
        "/app/hijacks_web/event_details/event_moas.js",
        "/app/hijacks_web/event_details/event_defcon.js",
        "/app/hijacks_web/event_details/event_edges.js",
    ];

    for (let i in script_paths) {
        $.ajax({
            url: script_paths[i],
            dataType: "script",
            async: false,
        });
    }
    $.ajax({
        dataType: "json",
        async: false,
        url: "/json/tags",
        success: function (data) {
            tags_info_dict = data;
        }
    });

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


function render_pfx_event_table(event_type, pfx_events, tr_skipped = false, tr_skip_reason = "", event_id = "", table_id = "#datatable", paging = true) {

    if (isEmpty(table_info_dict)) {
        load_event_scripts()
    }

    // render table based on event types
    let table = $(table_id).DataTable({
        data: pfx_events,
        paging: paging,
        searching: false,
        "columns": table_info_dict[event_type]["columns"],
        "columnDefs": table_info_dict[event_type]["columnDefs"],
    });

    if(tr_skipped){
        $(".no_tr").html("no; "+tr_skip_reason);
    }

    $('#datatable tbody').on('click', 'tr', function (e) {
        if(e.target.tagName === 'A'){
            return;
        }
        var data = table.row($(this)).data();
        let fingerprint = extract_pfx_event_fingerprint(data, event_type);
        let path = window.location.pathname.replace(/\/$/, "");
        window.open(`${path}/${fingerprint}`, "_self", false)
    });

}

function render_tr_availability(tr_results, pfx_event){
    if(tr_results.length > 0){
        let earliest_time = 0;
        if(isEmpty(tr_results[0].results)){
            return "<div class='no_tr'>no</div>"
        }
        for(let tr of tr_results[0]['results']){
            if(earliest_time ===0){
                earliest_time = tr['endtime'];
                continue
            }
            if(tr['starttime']<earliest_time){
                earliest_time = tr['endtime'];
            }
        }
        let res = "yes";
        let tr_time = (new Date(earliest_time*1000));
        if('finished_ts' in pfx_event && pfx_event['finished_ts'] !== null){
            let finish_time = Date.parse(pfx_event['finished_ts']);
            if(finish_time < tr_time){
                let diff_minutes = Math.floor((tr_time - finish_time)/1000/60);
                let explain = `traceroute performed ${diff_minutes} minutes after the event finished`;
                res += ` <span class="glyphicon glyphicon-exclamation-sign" data-toggle="tooltip" data-original-title="${explain}" data-html="true" data-placement="auto" aria-hidden="true"></span>`
            } else {
                let explain = `traceroute performed during the event`;
                res += ` <span class="glyphicon glyphicon-thumbs-up" data-toggle="tooltip" data-original-title="${explain}" data-html="true" data-placement="auto" aria-hidden="true"></span>`
            }
        }
        return res
    } else {
        return "<div class='no_tr'>no</div>"
    }
}

function render_impact(num_pfx, num_addrs) {
    let impact_str = "";
    if (num_pfx === 1) {
        impact_str += `${num_pfx} pfx `
    } else {
        impact_str += `${num_pfx} pfxs `
    }
    if (num_addrs === 1) {
        impact_str += `(${num_addrs} addr)`
    } else {
        impact_str += `(${num_addrs} addrs)`
    }
    return impact_str
}

function render_event_details_table(event_type, event) {
    $("#event-details-victim").html(
        render_origin_links(
            event["victims"], true, event['external']
        )
    );
    $("#event-details-attacker").html(
        render_origin_links(
            event["attackers"], true, event['external']
        )
    );
    $("#event-details-prefix").html(
        render_prefix_link(
            extract_largest_prefix(event["prefixes"])
        )
    );
    let [num_pfx, num_addrs] = extract_impact(event["prefixes"]);
    $("#event-details-impact").text(render_impact(num_pfx, num_addrs));
    $("#event-details-startts").text(unix_time_to_str(event["view_ts"]));
    $("#event-details-type").text(event_type_explain[event_type]);

    if (event["finished_ts"] === null) {
        $("#event-details-duration").text("ongoing");
        $("#event-details-endts").text("Unknown");
    } else {
        $("#event-details-duration").text(`${(event["finished_ts"] - event["view_ts"]) / 60} min`);
        $("#event-details-endts").text(unix_time_to_str(event["finished_ts"]));
    }

    let comment_html = "";
    event["inference"]["comments"].forEach(function(comment){
        comment_html += "<p>" + comment + "</p>"
    });
    $("#event-comments").html(comment_html);


    event_modal_info ["download_path"] = event["id"] + ".json";
    event_modal_info["json_raw_str"] = JSON.stringify(event, undefined, 4);
    // $(event_modal_info["content_id"]).html(syntaxHighlight(event_modal_info["json_raw_str"]));
    $(event_modal_info["content_id"]).html(renderjson.set_show_to_level(2)(event, 1));
    $(".full-event-modal-download").click(function () {
        let dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(event_modal_info["json_raw_str"]);
        var dlAnchorElem = document.getElementById(event_modal_info["anchorId"]);
        dlAnchorElem.setAttribute("href", dataStr);
        dlAnchorElem.setAttribute("download", event_modal_info["download_path"]);
        dlAnchorElem.click();
    });
}

/* Formatting function for row details - modify as you need */
function format_prefix_table(prefix) {
    // `d` is the original data object for the row
    let thead = '<table cellpadding="5" cellspacing="0" border="1" style="padding-left:50px;">';
    let tfoot = '</table>';
    let tbody = "";

    let records = whois_dict[prefix];
    if (records.length > 0) {
        records.forEach(function (record) {
            record.forEach(function (elem) {
                tbody += `<tr><td>${elem["key"]}</td><td>${elem["value"]}</td></tr>`
            });
            tbody += `<tr><td class="bottom-border"></td><td class="bottom-border"></td></tr>`
        });
    } else {
        tbody = "loading information ..."
    }
    return thead + tbody + tfoot;
}


blacklist_ases = new Set();

function render_origin_links(origin_lst, show_asn = false, external = null) {
    if(blacklist_ases.size === 0){
        $.ajax({
            dataType: "json",
            async: false,
            url: "/json/blacklist",
            success: function (data) {
                for(asn of data['blacklist']){
                    blacklist_ases.add(asn)
                };
            }
        });
    }
    let links = [];
    if (origin_lst === null || origin_lst.length === 0 || origin_lst[0] === "") {
        return "Unknown"
    }

    origin_lst.forEach(function (origin) {
        // links.push(`<a class="btn btn-default as-btn as-btn-${origin}" data-toggle="tooltip" title="" data-placement="top" href='http://as-rank.caida.org/asns/${origin}' target="_blank")> AS${origin} </a>`)
        let country_flag = render_country(origin, external);
        let as_html, as_tooltip;
        [as_html, as_tooltip] = render_origin(origin, external, show_asn);
        let blacklist_symbol = "";
        if(blacklist_ases.has(parseInt(origin))){
            blacklist_symbol = ` <span class="glyphicon glyphicon-warning-sign" data-toggle="tooltip" data-original-title="This AS is on blacklist" data-html="true" data-placement="auto" aria-hidden="true"></span>`
        }
        let external_links = "";
        if(show_asn){
            external_links = `<a class="link" href='http://as-rank.caida.org/asns/${origin}' target="_blank" data-toggle="tooltip" data-original-title="AS Rank" data-html="true" data-placement="auto" >ASRank</a>
,
<a class="link" href='https://stat.ripe.net/AS${origin}#tabId=at-a-glance' target="_blank" data-toggle="tooltip" data-original-title="RIPEstat" data-html="true" data-placement="auto" >RIPEstat</a>`
        }
        links.push(`<div>
<span class="as-country-${origin}" style="white-space:nowrap"> ${country_flag}</span>

<span data-toggle="tooltip" data-original-title="${as_tooltip}" data-html="true" data-placement="auto" >
${as_html}
</span>
${blacklist_symbol}

</div>`)
    });

    return links.join(" ")
}

function render_prefix_link(prefix) {
    let asns = "";
    let prefix_class = prefix.replace("/", "-").replace(/\./g, "-");
    // load_ripe_data(prefix, prefix_class);
    load_who_is(prefix);
    // return `<a class="btn btn-default pfx-btn-${prefix_class}" target="_blank" href='//stat.ripe.net/${prefix}#tabId=at-a-glance')> ${prefix}</a>`
    // return `<a class="link pfx-btn-${prefix_class}" target="_blank" href='//stat.ripe.net/${prefix}#tabId=at-a-glance')> ${prefix}</a>`
    return `<a class="link pfx-btn-${prefix_class}" target="_blank" href='//stat.ripe.net/widget/routing-history#w.resource=${prefix}')> ${prefix}</a>`
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

function load_traceroute_page(uuid) {
    let pfx_event = traceroute_hash[uuid];
    let path = window.location.pathname.replace(/\/$/, "");
    let path_segments = path.split("/");
    let event_type = path_segments[path_segments.length - 2];

    let fingerprint = extract_pfx_event_fingerprint(pfx_event, event_type);
    window.open(`${path}/${fingerprint}`, "_self", false)
}



function capitalizeFirstLetter(string) {
    return string.charAt(0).toUpperCase() + string.slice(1);
}

tag_type_ready = false;

function render_tag_name(tag) {
    let tag_name = tag.split("-")
        .map(
            function (x) {
                return capitalizeFirstLetter(x)
            })
        .join(" ")
    let link = `<a class="event_tag" target="_blank" href='/events/all?tags=${tag}')> ${tag_name}</a>`
    return link
}

type_label = {
    "yes": "label-danger",
    "no": "label-success",
    "na": "label-default",
};

function render_tags(event_type, tags) {
    entries = [];

    let tag_type = {};

    let tags_set = new Set(tags);
    for (let i in tags_info_dict["tr_worthy"]) {
        let entry = tags_info_dict["tr_worthy"][i];
        let worthiness = entry["worthy"];
        let comb = entry["tags"];
        let apply_to = entry["apply_to"];
        if ([...comb].filter(x => !tags_set.has(x)).length === 0) {
            if(!(apply_to.includes(event_type))){
                for(tag of comb){
                    tag_type[tag] = "na";
                }
                continue
            }
            // all items in comb is in tags set, the worthiness applies
            for (tag of comb) {
                if (!(tag in tag_type) || tag_type[tag] === "na") {
                    tag_type[tag] = worthiness;
                }
            }
        }
    }

    for (let i in tags) {
        let tag = tags[i];
        let label = "na";
        if (tag in tag_type) {
            label = tag_type[tag];
        } else {
            label = "na";
        }
        // if (!(tag in tag_type)) {
        //     entries.push(`<span style="color: purple; ">${tag}</span>`)
        // }
        let definition = tags_info_dict["definitions"][tag]["definition"];
        entries.push(`<span class="label ${type_label[label]}" style="font-size: 12px;" data-toggle='tooltip' title='${tag}: ${definition}'>${render_tag_name(tag)}</span></h4>`)
    }
    return entries.join(" &nbsp; ")
}

function render_country(origin, external){
    if(external == null || !('asrank' in external) || !(origin in external['asrank'])){
        return ""
    }
    return flag(external['asrank'][origin]['country']);
}

function render_origin(origin, external=null, show_asn=false){
    // load external information if exists
    if(external==null){
        return [`AS${origin}`, ""]
    }
    if('asrank' in external || 'hegemony' in external){
        let as_name = `AS${origin}`;
        if(origin in external['asrank']){
            as_name = process_as_name(external['asrank'][origin]);
            if(as_name === "Null") {
                as_name = `AS${origin}`;
            } else {
                // if(show_asn){
                //     // prepend asn
                //     as_name = `AS${origin} ${as_name}`;
                // }
                as_name = `AS${origin} ${as_name}`;
            }
        }

        return [as_name, _construct_tooltip(origin, external)]
    }
    return [`AS${origin}`, ""]
}
