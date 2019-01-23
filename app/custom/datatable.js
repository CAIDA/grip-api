let datatable = null;
let whois_dict = {};
let cidr_re = /^([0-9]{1,3}\.){3}[0-9]{1,3}(\/([0-9]|[1-2][0-9]|3[0-2]))?$/;
let cidr_loose_re = /^[0-9./]*$/;

function load_events_table(event_type) {
    $.extend(true, $.fn.dataTable.defaults, {
        "searching": false,
        "ordering": false,
    });
    $(document).ready(function () {

        datatable = $('#datatable').DataTable({
                "processing": true,
                "serverSide": true,
                "searching": false,
                "ordering": false,
                "ajax": {
                    "url": `/json/events/${event_type}`,
                },
                "columns": [
                    {title: "Event Type", "data": 'event_type'},
                    {title: "Fingerprint", "data": 'fingerprint'},
                    {title: "Prefix Events", "data": 'pfx_events_cnt'},
                    {title: "Status", "data": 'finished_ts'},
                    {title: "Time Stamp", "data": 'view_ts'},
                ],
                "columnDefs": [
                    {
                        "render": function (data, type, row) {
                            if (data === null) {
                                return "On-Going"
                            } else {
                                return `Finished at ${data}`
                            }
                        },
                        "targets": [3]
                    },
                ]
            }
        );

        $('#datatable tbody').on('click', 'tr', function () {
            var data = datatable.row($(this)).data();
            console.log("/json/event/id/" + data['id']);
            window.open("/event/" + data['event_type'] + "/" + data['id'], '_self', false);
        });
    });

    $("#query-btn").click(function () {
        let event_type = window.location.pathname.replace(/\/$/, "").split("/").pop();
        let times = $('#reportrange span').html().split(" - ");
        let url = `/json/events/${event_type}?ts_start=${times[0]}&ts_end=${times[1]}`;

        console.log(url);
        datatable.ajax.url(url).load();

    });

    $("#search-as-btn").click(function () {
        let asn = parseInt($("#search-as-input").val());
        if (!Number.isInteger(asn)) {
            alert("not an interger")
        } else {
            // alert(asn)
            let url = `/json/events/${event_type}?asn=${asn}`;
            console.log(url);
            datatable.ajax.url(url).load();
        }
    });


    $("#search-prefix-btn").click(function () {
        let prefix = $("#search-prefix-input").val().trim();
        if (!cidr_loose_re.test(prefix)){
            alert("not a prefix");
        } else {
            // alert(prefix)
            let url = `/json/events/${event_type}?prefix=${prefix}`;
            console.log(url);
            datatable.ajax.url(url).load();
        }
    });

    $("#search-as-input").keyup(function(event) {
        if (event.keyCode === 13) {
            $("#search-as-btn").click();
        }
    });

    $("#search-prefix-input").keyup(function(event) {
        if (event.keyCode === 13) {
            $("#search-prefix-btn").click();
        }
    });
}


var traceroute_hash = {};

function load_traceroute_page(uuid) {
    let pfx_event = traceroute_hash[uuid];
    let path = window.location.pathname.replace(/\/$/, "");
    let path_segments = path.split("/");
    let event_type = path_segments[path_segments.length - 2];

    let fingerprint = extract_pfx_event_fingerprint(pfx_event, event_type);
    window.open(`${path}/${fingerprint}`, "_self", false)
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

function load_event_details() {
    $(document).ready(function () {
        const event_id = get_event_id_from_url();
        const event_type = get_event_type_from_url();
        load_event_scripts();

        $.ajax({
            url: "/json/event/id/" + event_id,
            success: function (event) {
                render_pfx_event_table(event_type, event["data"]);
            }
        });
    })
}
