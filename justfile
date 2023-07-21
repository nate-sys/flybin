alias db := reset
alias s := serve

host := "localhost"
http-port := "8080"
nc-port := "9999"

# Rerun migrations 
reset:
    sqlx database reset

# Start the main server
serve: 
    cargo r --bin flybin-server

# create a paste
paste PASTE:
    echo {{PASTE}} | nc {{host}} {{nc-port}}

# get a paste : just get MySlug [password=something]
get SLUG *PASSARGS:
    curl -G \
        -d "{{PASSARGS}}" \
        http://{{host}}:{{http-port}}/{{SLUG}}

# get a syntax highlighted paste : just getlang rust MySlug [password=something]
getlang LANG SLUG *PASSARGS: 
    curl -G \
        -d "{{PASSARGS}}" \
        http://{{host}}:{{http-port}}/{{SLUG}}/{{LANG}}

# delete a paste : just delete MySlug MySecretToken
delete SLUG SECRET :
    curl -X DELETE -G \
        -d "secret={{SECRET}}" \
        http://{{host}}:{{http-port}}/{{SLUG}}


# lock a paste : just lock MySlug MySecretToken MyPassword
lock SLUG SECRET PASSWORD:
    curl -X POST -G \
        -d "password={{PASSWORD}}" \
        -d "secret={{SECRET}}" \
        http://{{host}}:{{http-port}}/{{SLUG}}

