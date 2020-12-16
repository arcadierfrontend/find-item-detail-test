<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Jotform</name>
   <tag></tag>
   <elementGuidId>1d5c2d6c-9058-477f-bd92-32e9c2006e2e</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='item-pop-box']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#item-pop-box</value>
      </entry>
   </selectorCollection>
   <selectorMethod>CSS</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>modal fade in</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>item-pop-box</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tabindex</name>
      <type>Main</type>
      <value>-1</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>role</name>
      <type>Main</type>
      <value>dialog</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
        
            
                    
                        
                        ×
                        
                    
                    
                        
                        
                        
                    
                    
                        var ifr = document.getElementById(&quot;JotFormIFrame-201660560316043&quot;);
                        if(window.location.href &amp;&amp; window.location.href.indexOf(&quot;?&quot;) > -1) {
                        var get = window.location.href.substr(window.location.href.indexOf(&quot;?&quot;) + 1);
                        if(ifr &amp;&amp; get.length > 0) {
                            var src = ifr.src;
                            src = src.indexOf(&quot;?&quot;) > -1 ? src + &quot;&amp;&quot; + get : src  + &quot;?&quot; + get;
                            ifr.src = src;
                        }
                        }
                        window.handleIFrameMessage = function(e) {
                        if (typeof e.data === 'object') { return; }
                        var args = e.data.split(&quot;:&quot;);
                        if (args.length > 2) { iframe = document.getElementById(&quot;JotFormIFrame-&quot; + args[(args.length - 1)]); } else { iframe = document.getElementById(&quot;JotFormIFrame&quot;); }
                        if (!iframe) { return; }
                        switch (args[0]) {
                            case &quot;scrollIntoView&quot;:
                            iframe.scrollIntoView();
                            break;
                            case &quot;setHeight&quot;:
                            iframe.style.height = args[1] + &quot;px&quot;;
                            break;
                            case &quot;collapseErrorPage&quot;:
                            if (iframe.clientHeight > window.innerHeight) {
                                iframe.style.height = window.innerHeight + &quot;px&quot;;
                            }
                            break;
                            case &quot;reloadPage&quot;:
                            window.location.reload();
                            break;
                            case &quot;loadScript&quot;:
                            var src = args[1];
                            if (args.length > 3) {
                                src = args[1] + ':' + args[2];
                            }
                            var script = document.createElement('script');
                            script.src = src;
                            script.type = 'text/javascript';
                            document.body.appendChild(script);
                            break;
                            case &quot;exitFullscreen&quot;:
                            if      (window.document.exitFullscreen)        window.document.exitFullscreen();
                            else if (window.document.mozCancelFullScreen)   window.document.mozCancelFullScreen();
                            else if (window.document.mozCancelFullscreen)   window.document.mozCancelFullScreen();
                            else if (window.document.webkitExitFullscreen)  window.document.webkitExitFullscreen();
                            else if (window.document.msExitFullscreen)      window.document.msExitFullscreen();
                            break;
                        }
                        var isJotForm = (e.origin.indexOf(&quot;jotform&quot;) > -1) ? true : false;
                        if(isJotForm &amp;&amp; &quot;contentWindow&quot; in iframe &amp;&amp; &quot;postMessage&quot; in iframe.contentWindow) {
                            var urls = {&quot;docurl&quot;:encodeURIComponent(document.URL),&quot;referrer&quot;:encodeURIComponent(document.referrer)};
                            iframe.contentWindow.postMessage(JSON.stringify({&quot;type&quot;:&quot;urls&quot;,&quot;value&quot;:urls}), &quot;*&quot;);
                        }
                        };
                        if (window.addEventListener) {
                        window.addEventListener(&quot;message&quot;, handleIFrameMessage, false);
                        } else if (window.attachEvent) {
                        window.attachEvent(&quot;onmessage&quot;, handleIFrameMessage);
                        }
                        
                    
                    Refresh
                    
                
            
        </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;item-pop-box&quot;)</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='item-pop-box']</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)=concat('id(', '&quot;', 'item-pop-box', '&quot;', ')')])[1]/following::div[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='`'])[2]/following::div[2]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[13]</value>
   </webElementXpaths>
</WebElementEntity>
