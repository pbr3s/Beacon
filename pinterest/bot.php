<?php
    require('vendor/autoload.php'); 
    use seregazhuk\PinterestBot\Factories\PinterestBot;
    $bot = PinterestBot::create();

    print_r($bot);
?>