const autocannon = require('autocannon')
const fs = require('fs')
const baseUrl="http://localhost:8080"

let abort=false

function showResults(err, result){
  if (abort===true) {
    console.log("Load test cancelled...")
    return
  }
  if (err) {
    console.error(err)
  }else{
    // console.log("Results received:", result)
    const fileName = `load_test_${Date.now()}.json`
    fs.writeFileSync(fileName, JSON.stringify(result))
    console.log("Saved to file:", fileName)
  }
}

const todoItem = {
  "title":"Todo item title",
  "checked": false
}

const loadTest = autocannon({
  title:"actix-todo",
  url: baseUrl,
  connections:10,
  duration: 3,
  requests:[{
      method:'GET',
      path:'/',
    },{
      method: 'POST',
      path:'/todos/1/items',
      headers:{
        'content-type':'application/json',
        'autohorization':'Bearer FAKE_JWT_KEY'
      },
      body:JSON.stringify(todoItem)
    }
  ]
},showResults)

process.once('SIGINT',()=>{
  abort = true
  loadTest.stop()
})

autocannon.track(loadTest)