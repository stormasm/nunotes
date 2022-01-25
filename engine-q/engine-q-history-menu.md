
https://github.com/nushell/engine-q/pull/846

Introduces a history menu for engine q
it can be triggered with Ctrl+a

The menu presents the history in pages that can be navigated with down or up arrows (tab and shift tab)
to see the next page use ctrl+a and ctr + shift + a for previous page
All of these events can be changed in config file

To configure the menu in the config use

 history_config: {
   page_size: 10
   selector: ":"                                                                                                                          
   text_style: dark_gray
    selected_text_style: green_reverse
  }
the selector is a character that when typed with a number after it will select that input. e.g. :4 will select that row
