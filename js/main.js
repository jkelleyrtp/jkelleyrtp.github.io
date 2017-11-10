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
      } else if (scrollTop < 100) {
        $('#switcher').removeClass('scrolled-nav');


        //alert("hello world!")

      }

    });



});
