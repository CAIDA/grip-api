function load_ripe_data(prefix, prefix_class) {
    $.ajax({
        url: `https://stat.ripe.net/data/prefix-overview/data.json?resource=${prefix}`,
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
            url: `https://stat.ripe.net/data/whois/data.json?resource=${prefix}`,
            success: function (pfx_whois) {
                // let authorities = pfx_whois["data"]["authorities"].map(v => v.toLowerCase());
                // authorities.push("radb");
                let records = pfx_whois["data"]["records"];
                console.log(records);
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

function load_origins_asrank(origin_lst, style) {
    origin_lst.forEach(function (origin) {
            load_origin_asrank(origin, style)
        }
    );
}

function load_origin_asrank(origin, style=1) {
    $.ajax({
        url: `http://as-rank.caida.org/api/v1/asns/${origin}`,
        success: function (asorg) {
            if (asorg["data"] != null) {
                let as_name = process_as_name(asorg["data"]);
                if(style === 1){
                    $(`.as-btn-${origin}`).each(function () {
                        // $(this).html(`AS${origin} ${asorg["data"]["country"]} ${as_name}`);
                        // $(this).attr("title", `${asorg["data"]["country_name"]}, ${asorg["data"]["org"]["name"]}`)
                        if(as_name === "Null"){
                            as_name = `AS${origin}`
                        }
                        $(this).html(`${as_name} (${asorg["data"]["country"]}) `);
                        // pity, the following looks nice if only a few badges is on
                        // $(this).html(`${as_name} <span class="badge">${asorg["data"]["country"]}</span> `);
                        $(this).attr("title", `AS${origin}, ${asorg["data"]["org"]["name"]}, ${asorg["data"]["country_name"]}, `)
                    });
                } else if (style === 2){
                    $(`.as-btn-${origin}`).each(function () {
                        // $(this).html(`AS${origin} ${asorg["data"]["country"]} ${as_name}`);
                        // $(this).attr("title", `${asorg["data"]["country_name"]}, ${asorg["data"]["org"]["name"]}`)
                        if(as_name === "Null"){
                            as_name = `AS${origin}`
                        }
                        $(this).html(`AS${origin} ${as_name} (${asorg["data"]["country"]}) `);
                        // pity, the following looks nice if only a few badges is on
                        // $(this).html(`${as_name} <span class="badge">${asorg["data"]["country"]}</span> `);
                        $(this).attr("title", `AS${origin}, ${asorg["data"]["org"]["name"]}, ${asorg["data"]["country_name"]}, `)
                    });

                }
            }
        },
    })
}
