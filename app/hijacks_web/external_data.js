let whois_dict = {};
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

as_info = {'hegemony':{}, 'asrank':{}};

function _construct_tooltip(asn, external){
    let table_str = "";
    if("asrank" in external && asn in external['asrank']){
        // load as org information
        let asorg = external["asrank"][asn];
        if("org" in asorg && "name" in asorg["org"]){
            // the `if` statement makes sure the data exists before refer to it
            asorg["org"]["name"] = asorg["org"]["name"].replace(/"/g, "");
            table_str+= `
                ASN: ${asorg["id"]} <br/>
                Name: ${asorg["org"]["name"]} <br/>
                Country: ${asorg["country_name"]} <br/>
                Rank: ${asorg["rank"]} <br/>
            `
        }
    }

    if("hegemony" in external && asn in external['hegemony']){
        let hege_score = external["hegemony"][asn];
        table_str+= `Hegemony score: ${hege_score} <br/>`;
    }

    return table_str
}
