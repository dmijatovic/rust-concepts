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

const todoList={
  "title":"New todo list"
}

const todoItem = {
  "title":"Todo item",
  "checked": false
}

const loadTest = autocannon({
  title:"actix-todo",
  url: baseUrl,
  connections:50,
  duration: 20,
  requests:[{
      method:'GET',
      path:'/',
    },{
      method:'POST',
      path:'/todos',
      headers:{
        'content-type':'application/json',
        'autohorization':'Bearer FAKE_JWT_KEY'
      },
      body:JSON.stringify(todoList),
      onResponse:(status, body, context)=>{
        if (status === 200) {
          const resp = JSON.parse(body)
          context['list_id'] = resp['id']
        }
      }
    },{
      method: 'POST',
      setupRequest:(req, context)=>({
        ...req,
        path:`/todos/${context['list_id']}/items`,
        headers:{
          'content-type':'application/json',
          'autohorization':'Bearer FAKE_JWT_KEY'
        },
        body:JSON.stringify(todoItem)
      })
    },{
      method: 'GET',
      setupRequest:(req, context)=>({
        ...req,
        path:`/todos/${context['list_id']}/items`,
        headers:{
          'content-type':'application/json',
          'autohorization':'Bearer FAKE_JWT_KEY'
        }
      })
    }
  ]
},showResults)

process.once('SIGINT',()=>{
  abort = true
  loadTest.stop()
})

autocannon.track(loadTest)