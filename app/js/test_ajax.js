function load_ten() {
    $.ajax({
        type: "GET",
        url: '/getten',
        success: function (data) {
            $('#test_ajax').html(data["value"]);
        }

    });
}