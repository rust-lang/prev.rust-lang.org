include = function() {

  // save references to save a few bytes
	var args = arguments;
	var doc = document;

	var toLoad = args.length; // load this many scripts
  var lastArgument = args[toLoad - 1];
	var hasCallback = lastArgument.call; // is the last arg a callback?
	if (hasCallback) {
		toLoad --;
	}

	function onScriptLoaded() {
    var readyState = this.readyState; // we test for "complete" or "loaded" if on IE
    if (!readyState || /ded|te/.test(readyState)) {
      toLoad --;
      if (!toLoad && hasCallback) {
        lastArgument();
      }
    }
	}

	var script;
	for (var i = 0; i < toLoad; i ++) {

		script = doc.createElement('script');
		script.src = arguments[i];
    script.async = true;
		script.onload = script.onerror = script.onreadystatechange = onScriptLoaded;
		(
			doc.head ||
			doc.getElementsByTagName('head')[0]
		).appendChild(script);

	}

};
