function load_ripe_data(prefix, prefix_class) {
    $.ajax({
        url: `//stat.ripe.net/data/prefix-overview/data.json?resource=${prefix}`,
        success: function (pfx_info) {
            let asns = pfx_info["data"]["asns"].map(function (elem) {
                return "AS" + elem["asn"]
            }).join(" ");
            $(`.pfx-btn-${prefix_class}`).html(`${prefix} ${asns}`)
        }
    });
}


function load_who_is(prefix) {
    if (!(prefix in whois_dict)) {
        whois_dict[prefix] = "";
        $.ajax({
            url: `//stat.ripe.net/data/whois/data.json?resource=${prefix}`,
            success: function (pfx_whois) {
                // let authorities = pfx_whois["data"]["authorities"].map(v => v.toLowerCase());
                // authorities.push("radb");
                let records = pfx_whois["data"]["records"];
                if (records.length === 1) {
                    whois_dict[prefix] = records;
                    return
                }

                let filtered_records = [];
                records.forEach(function (record) {
                    let match = false;
                    record.some(function (elem) {
                        if (elem["key"] === "inetnum" || elem["key"] === "CIDR") {
                            match = true;
                            return true
                        }
                    });
                    if (match) {
                        filtered_records.push(record);
                    }
                });
                if (filtered_records.length === 0) {
                    whois_dict[prefix] = records
                } else {
                    whois_dict[prefix] = filtered_records;
                }
            }
        });
    }
}

function load_origins_info(origin_lst, style) {
    origin_lst.forEach(function (origin) {
            load_origin_info(origin, style)
        }
    );
}

function _construct_asrank_table(asn){
    if(!(asn in as_info)){
        return ""
    }

    let table_str = "";

    if("asorg" in as_info[asn]){
        // load as org information
        let asorg = as_info[asn]["asorg"];
        table_str+= `
            ASN: ${asorg["data"]["id"]} <br/>
            Name: ${asorg["data"]["org"]["name"]} <br/>
            Country: ${asorg["data"]["country_name"]} <br/>
            Rank: ${asorg["data"]["rank"]} <br/>
            Cone size: ${asorg["data"]["cone"]["asns"]} <br/>
            Prefixes: ${asorg["data"]["cone"]["prefixes"]} <br/>
        `
    }

    if("hegemony" in as_info[asn]){
        let hegemony = as_info[asn]["hegemony"];
        if(hegemony["count"]>0){
            let hege_score = hegemony["results"][0]["hege"];
            table_str+= `Hegemony score: ${hege_score} <br/>`;
        }
    }

    return table_str
}

function render_country(asorg) {
    let country_code = asorg["data"]["country"];
    return flag(country_code)
}

function load_origin_info(origin, style=1){
    // initialize tooltip then change the title later it later
    $(`.as-btn-${origin}`).each(function () {
        $(this).tooltip({
            title: "",
            html: true,
            placement: "auto"
        });
    });
    load_origin_asrank(origin, style);
    load_origin_hegemony(origin);
}

let hegemony_query_time = moment().subtract(2, 'days').format('YYYY-MM-DDTHH:00');

function load_origin_hegemony(origin){
    $.ajax({
        // url: `/json/hegemony/${origin}`,
        url: `https://ihr.iijlab.net/ihr/api/hegemony/?originasn=0&af=4&timebin=${hegemony_query_time}&format=json&asn=${origin}`,
        async: true,
        success: function (hegemony) {
            if(!(origin in as_info)){
                as_info[origin] = {}
            }
            as_info[origin]["hegemony"]=hegemony;
            $(`.as-btn-${origin}`).each(function () {
                $(this) .attr('data-original-title',_construct_asrank_table(origin))
            })
        }
    })

}

as_info = {};

function load_origin_asrank(origin, style) {
    $.ajax({
        // url: `/json/asrank/${origin}`,
        url: `/json/asrank/${origin}`,
        async: true,
        success: function (asorg) {
            if (asorg["data"] != null) {
                let as_name = process_as_name(asorg["data"]);
                if(style === 1){
                    $(`.as-btn-${origin}`).each(function () {
                        if(as_name === "Null"){
                            as_name = `AS${origin}`;
                            $(this).tooltip({
                                title: "Unknown",
                                html: true,
                                placement: "auto"
                            });
                        } else{
                            if(!(origin in as_info)){
                                as_info[origin] = {}
                            }
                            as_info[origin]["asorg"]=asorg;
                            $(this).attr('data-original-title',_construct_asrank_table(origin));
                        }
                        $(this).html(`${as_name}`);
                    });
                    $(`.as-country-${origin}`).each(function () {
                        $(this).html(`${render_country(asorg)}`);
                    })

                } else if (style === 2){
                    $(`.as-btn-${origin}`).each(function () {
                        if(as_name === "Null"){
                            as_name = `AS${origin}`;
                            $(this).tooltip({
                                title: "Unknown",
                                html: true,
                                placement: "auto"
                            });
                            $(this).html(`AS${origin}`);
                        } else {
                            if(!(origin in as_info)){
                                as_info[origin] = {}
                            }
                            as_info[origin]["asorg"]=asorg;
                            $(this).attr('data-original-title',_construct_asrank_table(origin));
                            $(this).html(`AS${origin} ${as_name}`);
                        }
                    });
                    $(`.as-country-${origin}`).each(function () {
                        $(this).html(`${render_country(asorg)}`);
                    })
                }
            } else {
                // as org information not found
                $(`.as-btn-${origin}`).each(function () {
                    let as_name = `AS${origin}`;
                    $(this).html(`${as_name}`);
                    $(this).tooltip({
                        title: "Unknown",
                        html: true,
                        placement: "auto"
                    });
                });
            }
        },
    })
}
