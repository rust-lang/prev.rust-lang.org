// ECMAScript 6 Backwards compatability
if (typeof String.prototype.startsWith != 'function') {
  String.prototype.startsWith = function(str) {
    return this.slice(0, str.length) == str;
  };
}

// Fetching DOM items
var activeCode = document.getElementById("active-code");
var staticCode = document.getElementById("static-code");
var runButton = document.getElementById("run-code");
var resultDiv = document.getElementById("result");

// Background colors for program result on success/error
var successColor = "#E2EEF6";
var errorColor = "#F6E2E2";

// Error message to return when there's a server failure
var errMsg = "The server encountered an error while running the program.";

// Stores ACE editor markers (highights) for errors
var markers = [];

// JS exists, display ACE editor
staticCode.style.display = "none";
activeCode.style.display = "block";

// Setting up ace editor
var editor = ace.edit("editor");
var Range = ace.require('ace/range').Range;
editor.setTheme("ace/theme/chrome");
editor.getSession().setMode("ace/mode/rust");
editor.getSession().setTabSize(2);
editor.setShowPrintMargin(false);
editor.renderer.setShowGutter(false);

// Dispatches a XMLHttpRequest to the Rust playpen, running the program, and
// issues a callback to `callback` with the result (or null on error)
function runProgram(program, callback) {
  var req = new XMLHttpRequest();
  var data = JSON.stringify({
    version: "master",
      optimize: "2",
      code: program
  });

  <!-- console.log("Sending", data); -->
    req.open('POST', "http://playtest.rust-lang.org/evaluate.json", true);
  req.onload = function(e) {
    if (req.readyState === 4 && req.status === 200) {
      var result = JSON.parse(req.response).result;
      // Need server support to get an accurate version of this.
      var isSuccess = (result.indexOf("error:") === -1);
      callback(isSuccess, result);
    } else {
      callback(false, null);
    }
  };

  req.onerror = function(e) {
    callback(false, null);
  }

  req.setRequestHeader("Content-Type", "application/json");
  req.send(data);
}

// The callback to runProgram
function handleResult(success, message) {
  var message = message.replace(/<anon>/g, '');
  var message = message.replace(/(?:\r\n|\r|\n)/g, '<br />');

  // Dispatch depending on result type
  if (result == null) {
    resultDiv.style.backgroundColor = errorColor;
    resultDiv.innerHTML = errMsg;
  } else if (success) {
    handleSuccess(message);
  } else {
    handleError(message);
  }
}

// Called on successful program run
function handleSuccess(message) {
  resultDiv.style.backgroundColor = successColor;
  resultDiv.innerHTML = message;
}

// Called on unsuccessful program run. Detects and prints errors in program
// output and highlights relevant lines and text in the code.
function handleError(message) {
  resultDiv.style.backgroundColor = errorColor;

  // Getting list of ranges with errors
  var lines = message.split("<br />");
  var ranges = parseError(lines);

  // Cleaning up the message: keeps only relevant error output
  var cleanMessage = lines.map(function(line) {
    var errIndex = line.indexOf("error: ");
    if (errIndex !== -1) {
      return line.slice(errIndex);
    }
    return "";
  }).filter(function(line) {
    return line !== "";
  }).join("<br />");

  // Setting message
  resultDiv.innerHTML = cleanMessage;

  // Highlighting the lines
  markers = ranges.map(function(range) {
    return editor.getSession().addMarker(range, "ace-error-line", "fullLine", false);
  });

  // Highlighting the specific text
  markers = markers.concat(ranges.map(function(range) {
    return editor.getSession().addMarker(range, "ace-error-text", "text", false);
  }));
}

// Parses an error message returning a list of ranges (row:col, row:col) where
// erors in the code have occured.
function parseError(lines) {
  var ranges = [];
  for (var i in lines) {
    var line = lines[i];
    if (line.startsWith(":") && line.indexOf(": ") !== -1) {
      var parts = line.split(/:\s?|\s+/, 5).slice(1, 5);
      var ip = parts.map(function(p) { return parseInt(p, 10) - 1; });
      <!-- console.log("line:", line, parts, ip); -->
        ranges.push(new Range(ip[0], ip[1], ip[2], ip[3]));
    }
  }

  return ranges;
}

// Registering handler for run button click
runButton.addEventListener("click", function(ev) {
  resultDiv.style.display = "block";
  resultDiv.innerHTML = "Running...";

  // clear previous markers, if any
  markers.map(function(id) { editor.getSession().removeMarker(id); });

  // Get the code, run the program
  var program = editor.getValue();
  runProgram(program, handleResult);
});
