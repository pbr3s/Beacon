<?php
    error_reporting(E_ALL);

    require('vendor/autoload.php'); 
    use seregazhuk\PinterestBot\Factories\PinterestBot;

    // $username = $argv[1];
    // $password = $argv[2];

    $bot = PinterestBot::create();
    $bot->getHttpClient()->setCookiesPath("./cookies");

    echo("--------------------------\n\n\nVamos fazer a busca de informações de um pin");
    $pins = $bot->pins->search('cats')->toArray();
    print_r($pins);
?>