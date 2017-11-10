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

        console.log(all_entires);
      } else if (scrollTop < 100) {
        $('#switcher').removeClass('scrolled-nav');


        //alert("hello world!")

      }

    });



});





var all_entires = $.getJSON("https://spreadsheets.google.com/feeds/list/1691V8YfwWH3LAogJHUOsThCErwi-5qnY5a5sMGfIo7M/od6/public/values?alt=json", function(data) {
  // All entries

  var entries = data.feed.entry;

  var projects = entries.filter(function(a){return a.gsx$type.$t =="Project"});

  var activities = entries.filter(function(a){return a.gsx$type.$t =="Activities"});
console.log(projects);
return entries;
  //console.log(data.feed)
});
