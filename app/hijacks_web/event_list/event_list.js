function load_events_table(only_benign=false) {

    const elems = window.location.pathname.replace(/\/$/, "").split("/");
    if(elems.length !==3){
        return
    }
    let nature = get_event_nature_from_url();
    let event_type = get_event_type_from_url();
    let misconf_type = "";
    if(nature === "misconf" && !(event_type in ["all", "moas", "submoas", "defcon", "edges"])){
        misconf_type = event_type;
        event_type = "all";
    }

    let frame_type = event_type;
    if(frame_type==="all"){
        frame_type = "overall";
    }
    $.extend(true, $.fn.dataTable.defaults, {
        "searching": false,
        "ordering": false,
    });
    $(document).ready(function () {
        $('body').tooltip({selector: '[data-toggle="tooltip"]'});
        let url = `/json/events/${event_type}?`;
        let search_text = [];
        let start_ts = "";
        let end_ts = "";
        if(!params.has("")){
            params.forEach(function(value, key, map){
                if(!key.startsWith("ts_")) {
                    // strip existing searching ranges
                    url += `${key}=${value}&`;
                }else{
                    if(key==="ts_start"){
                        start_ts = value
                    } else if(key==="ts_end"){
                        end_ts = value
                    }
                }
                if(key === "asn"){
                    search_text.push("AS"+value)
                } else if (key === "prefix") {
                    search_text.push(value)
                } else if (key === "tags") {
                    search_text.push(value)
                }
            });
        }
        if(start_ts!==""){
            $('#reportrange span').html(start_ts + ' - ' + end_ts);
        }
        $("#search-box").val(search_text.join(" "));
        let times = $('#reportrange span').html().split(" - ");
        if(Date.parse(times[0]) !==null){
            url += `ts_start=${times[0]}&ts_end=${times[1]}`;
        }

        let num_plot_elem = document.getElementById("num_plot");

        if(nature === "suspicious") {
            $("#stats-frame").html(`<iframe src="//ioda.caida.org/public/hijacks-trworthy-${frame_type}" width="100%" height="500" frameborder="0"></iframe>`);
            url += "&min_susp=80";
            if(num_plot_elem != null){
                num_plot_elem.style.display = "none";
            }
        } else if(nature === "benign"){
            url += "&max_susp=20";
            if(num_plot_elem != null){
                num_plot_elem.style.display = "none";
            }
        } else if (nature === "grey") {
            if(num_plot_elem != null){
                num_plot_elem.style.display = "none";
            }
            url += "&max_susp=79&min_susp=21";
        } else if(nature === "misconf") {
            if(num_plot_elem != null){
                num_plot_elem.style.display = "none";
            }
            url += `&misconf=true&misconf_type=${misconf_type}`;
        } else if(nature === "all") {
            if(num_plot_elem != null){
                num_plot_elem.style.display = "none";
            }
        }



        url = url.replace(/[?&]$/i, "");
        console.log("query sent to " + url);
        let event_type_count = {
            "moas": 0,
            "submoas": 0,
            "defcon": 0,
            "edges": 0,
        };
        let datatable = $('#datatable').DataTable({
                "processing": true,
                "serverSide": true,
                "searching": false,
                "ordering": false,
                "pageLength": 10,
                "ajax": {
                    "url": url,
                },
                "columns": [
                    {title: "Potential Victim", "data": 'inference.victims'},
                    {title: "Potential Attacker", "data": 'inference.attackers'},
                    {title: "Largest Prefix", "data": 'prefixes'},
                    {title: "# Prefix Events", "data": 'prefixes'},
                    {title: "Start Time", "data": 'view_ts'},
                    {title: "Duration", "data": 'duration'},
                    {title: "Type", "data": 'event_type'},
                ],
                "columnDefs": [
                    {
                        "render": function (data, type, row) {
                            // let victims = extract_victims(data[0], row["event_type"]);
                            let victims = data;
                            if(victims===undefined){
                                victims = row["victims"];
                            }
                            let links = "";
                            if(victims.length>0){
                                links = render_origin_links(victims.slice(0,2), false, row['external']);
                                if(victims.length>1){
                                    // links+= `<div>(${victims.length-1} more)</div>`
                                }
                            }
                            return links;
                        },
                        "targets": [0]
                    },
                    {
                        "render": function (data, type, row) {
                            // let attackers = extract_attackers(data[0], row["event_type"]);
                            let attackers = data;
                            if(attackers===undefined){
                                attackers = row["attackers"];
                            }
                            let links = "";
                            if(attackers.length>0){
                                links = render_origin_links(attackers.slice(0,2), false, row['external']);
                                if(attackers.length>1){
                                    links+= `<div>(${attackers.length-1} more)</div>`
                                }
                            }
                            return links;
                        },
                        "targets": [1]
                    },
                    {
                        "width": "8em",
                        "render": function (data, type, row) {
                            return extract_largest_prefix(data)
                        },
                        "targets": [2]
                    },
                    {
                        "width": "12em",
                        "render": function (data, type, row) {
                            [num_pfx, num_addrs] = extract_impact(data);
                            return render_impact(num_pfx, num_addrs)
                        },
                        "targets": [3]
                    },
                    {
                        "width": "10em",
                        "render": function (data, type, row) {
                            return data.split("T").join("  ")
                        },
                        "targets": [4]
                    },
                    {
                        "width": "6em",
                        "render": function (data, type, row) {
                            if (data === null) {
                                return "ongoing"
                            } else {
                                // let start_ts = Date.parse(row["view_ts"]);
                                // let end_ts = Date.parse(data);
                                // let duration = (end_ts-start_ts)/1000/60;
                                let duration = data/60;
                                if(duration < 0 ){
                                    // FIXME: figure out why duration would be below 0
                                    duration = 0;
                                }
                                return `${duration} min`
                            }
                        },
                        "targets": [5]
                    },
                    {
                        "width": "13em",
                        "render": function (data, type, row) {
                            event_type_count[row["event_type"]]++;
                            return event_type_explain[row["event_type"]];
                        },
                        "targets": [6]
                    },
                ],
            }
        );

        datatable.on( 'draw.dt', function () {
            if(event_type_count["moas"] + event_type_count["submoas"] ===0){
                datatable.column( 1 ).visible(false);
            }
        } );

        datatable.on( 'processing.dt', function () {
            let page_number = datatable.page.info()['page'];
            if(window.location.hash){
                let hash_number = parseInt(window.location.hash.split("#")[1]);
                if(page_number!==hash_number){
                    console.log(`change page now from ${page_number} to ${hash_number}`);
                    datatable.page(hash_number).draw(false);
                }
            }
        });

        datatable.on( 'page.dt', function () {
            let info = datatable.page.info();
            window.location.hash = info['page'];
            datatable.draw(false);
        } );

        $('#datatable tbody').on('click', 'tr', function (e) {
            if(e.target.tagName === 'A'){
                return;
            }
            var data = datatable.row($(this)).data();
            let base = window.location.pathname.split("/")[1];
            // if(base === "events_benign" || base === "events_grey"){
            //     base = "events"
            // }
            if(base==='hi3'){
                base = "hi3"
            } else {
                base = "events"
            }
            window.open(`/${base}/` + data['event_type'] + "/" + data['id'], '_self', false);
        });
    }); // end of document.ready

    $("#search-btn").click(function () {
        let fields = $("#search-box").val().trim().split(" ");
        if(fields.length > 2){
            alert("allow maximum searching for 1 ASN and 1 prefix")
            return;
        }
        let asn = "";
        let prefix = "";
        let tags = [];
        let ready = false;
        console.log(fields);
        for(let i in fields){
            let v = fields[i].trim().toLowerCase();
            // check if it's a prefix
            if(cidr_loose_re.test(v)){
                prefix = v;
                ready = true;
            }

            // check if it's a tag
            if((/^[a-zA-Z\-]+$/).test(v)){
                tags.push(v);
                ready = true;
            }

            // check if it's an as number
            v = v.replace(/as/i,"");
            if((/^[0-9]+$/).test(v)){
                // it's a asn
                asn = v;
                ready = true;
            }
        }
        if(!ready){
            // alert("not enough search parameters");
            console.log("not enough search parameters");
            return;
        }
        let url = `/${window.location.pathname.split("/")[1]}/${event_type}?`;
        if(prefix!==""){
            url+=`prefix=${prefix}&`;
        }
        if(asn!==""){
            url+=`asn=${asn}&`;
        }
        if(tags.length>0){
            url+=`tags=${tags.join(",")}`
        }
        url = url.replace(/[?&]$/i, "");
        window.open(url, '_self', false);
    });


    $("#search-box").keyup(function(event) {
        if (event.keyCode === 13) {
            $("#search-btn").click();
        }
    });
}
