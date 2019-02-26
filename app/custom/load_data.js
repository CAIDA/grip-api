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

function load_origins_asrank(origin_lst, style) {
    origin_lst.forEach(function (origin) {
            load_origin_asrank(origin, style)
        }
    );
}

function _construct_asrank_table(asorg, hege_score, simple=false){
    if(simple){
        return `
ASN: ${asorg["data"]["id"]} <br/>
Name: ${asorg["data"]["org"]["name"]} <br/>
Country: ${asorg["data"]["country_name"]} <br/>
Rank: ${asorg["data"]["rank"]} <br/>
Hegemony score: ${hege_score} <br/>
Cone size: ${asorg["data"]["cone"]["asns"]} <br/>
Prefixes: ${asorg["data"]["cone"]["prefixes"]} <br/>
    `

    } else {

    return `
        <table>
            <tr>
                <td>ASN: </td>
                <td> ${asorg["data"]["id"]} </td>
            </tr>
            <tr>
                <td>name: </td>
                <td> ${asorg["data"]["org"]["name"]} </td>
            </tr>
            <tr>
                <td>country: </td>
                <td> ${asorg["data"]["country_name"]} </td>
            </tr>
            <tr>
                <td>rank: </td>
                <td> ${asorg["data"]["rank"]} </td>
            </tr>
            <tr>
                <td>cone size: </td>
                <td> ${asorg["data"]["cone"]["asns"]} </td>
            </tr>
            <tr>
                <td>prefixes: </td>
                <td> ${asorg["data"]["cone"]["prefixes"]} </td>
            </tr>
        </table>
    `
    }
}

function render_country(asorg) {
    let country_code = asorg["data"]["country"];
    // return country_code+flag(country_code)
    return flag(country_code)
}

function load_hegemony(asn){

    // fixme: use event's corresponding time to load hegemony score
    let hege_score = 0;
    let url = `https://ihr.iijlab.net/ihr/api/hegemony/?originasn=0&af=4&timebin=${current_time}&format=json&asn=${asn}`;
    $.ajax({
        url: url,
        async: false,
        success: function(data) {
            if(data["count"]===0){
                return
            }
            hege_score = data["results"][0]["hege"];
            console.log(hege_score);
        },
    });

    return hege_score
}

function load_origin_asrank(origin, style=1) {
    $.ajax({
        url: `/json/asrank/${origin}`,
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
                            let hege_score = load_hegemony(origin);
                            $(this).tooltip({
                                title: _construct_asrank_table(asorg, hege_score, true),
                                html: true,
                                placement: "auto"
                            });
                        }
                        $(this).html(`${as_name}`);
                    });
                    $(`.as-country-${origin}`).each(function () {
                        $(this).html(`${render_country(asorg)}`);
                    })

                } else if (style === 2){
                    $(`.as-btn-${origin}`).each(function () {
                        // $(this).html(`AS${origin} ${asorg["data"]["country"]} ${as_name}`);
                        // $(this).attr("title", `${asorg["data"]["country_name"]}, ${asorg["data"]["org"]["name"]}`)
                        if(as_name === "Null"){
                            as_name = `AS${origin}`;
                            $(this).tooltip({
                                title: "Unknown",
                                html: true,
                                placement: "auto"
                            });
                            $(this).html(`AS${origin}`);
                        } else {
                            let hege_score = load_hegemony(origin);
                            $(this).tooltip({
                                title: _construct_asrank_table(asorg, hege_score, simple=true),
                                html: true,
                                placement: "auto"
                            });
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
