var JdC = JdC || {};

JdC.HttpRequest = function(url, method, data, user, callback) {
  if (user != null) {
	  if (data == null) {data = {};}
	  data['access_token'] = user.getAuthResponse().id_token;
  }
  
  var xhr = new XMLHttpRequest();
  if ((method == "GET") && (data != null)) {
	  var parms = "";
	  Object.keys(data).forEach(function(key) {
		  parms += ((parms == "") ? "" : "&")+encodeURIComponent(key)+"="+encodeURIComponent(data[key]);
		  });
	  url += "?" + parms;
  }
  xhr.open(method, url);

  xhr.onreadystatechange = function () {
    if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
      callback(xhr.responseText);
    } else if (this.readyState === XMLHttpRequest.DONE) {
    	callback("{}");
    }
  };

  if (method == "GET") {
	  xhr.send();
  } else {
	  xhr.send(data);
  }
};
