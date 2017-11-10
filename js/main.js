/*
 Jonathan Kelley


 Gsync - Portfolio Project


 */

$(window).load(function() {

    /*
    * Loading
    */

    $('.loading-part').fadeOut();

    $(window).scroll(function(){
      scrollTop = $(window).scrollTop();
       $('.counter').html(scrollTop);

      if (scrollTop >= 100) {
        $('#switcher').addClass('scrolled-nav');

        //console.log(all_entires);

      } else if (scrollTop < 100) {
        $('#switcher').removeClass('scrolled-nav');


        //alert("hello world!")

      }

    });



});




function generate_rows(data){
  var row_count = Math.ceil( data.length / 3.0 );
  console.log(row_count);

  var row_names = [];

  for (row_id = 1; row_id < (row_count + 1 );  row_id++ ) {
    row_names.push("row"+row_id.toString());
  }

  row_names.forEach(function(element){
    var output = "<div class='row' id = '";

    $( "#row-display" ).append(output + element + "'> </div>");
    //document.getElementById('row-display')
  });

  var current_id = 1;

  data.forEach(function(a){
    var imgsrc = "images/" + a.gsx$primaryimage.$t;
    var title = a.gsx$name.$t;
    var description = a.gsx$description.$t;
    var href = "jkelleyrtp.github.io/portfolio/"

    var output = `<div class="variant col-xs-12 col-sm-4 col-lg-4">
                    <a href="${href}" target="_blank">
                      <img src="${imgsrc}" alt="" class="img-fluid">
                      <h3>${title}</h3>
                    </a>
                  </div>`;

    console.log("row"+  Math.ceil( current_id/3.0  ).toString());

    console.log(Math.ceil( current_id/3.0  ));


    console.log(output);
    $( "#row"+  Math.ceil( current_id/3.0  ).toString()   ).append(output);

    current_id += 1;



  });




    console.log(row_names);
}






//var data = [];



var all_entires = $.getJSON("https://spreadsheets.google.com/feeds/list/1691V8YfwWH3LAogJHUOsThCErwi-5qnY5a5sMGfIo7M/od6/public/values?alt=json", function(data) {
  // All entries

  //data = console.log(data.feed.entry);

  //data = data.feed.entry;

  var entries = data.feed.entry;

  var projects = entries.filter(function(a){return a.gsx$type.$t =="Project"});

  var activities = entries.filter(function(a){return a.gsx$type.$t =="Activities"});

  generate_rows(entries);
});
