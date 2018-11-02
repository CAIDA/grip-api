function load_ten()
{
    $.ajax({
        type: "GET",
        url: 'http://localhost:8000/getten',
        success: function(data) {
            $('#test_ajax').html(data["value"]);
        }

    });

}
