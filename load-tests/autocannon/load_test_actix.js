const autocannon = require('autocannon')
const utils = require('./utils')
const requests = require('./requests')

let abort=false

function saveResults(err, result){
  if (abort===true) {
    console.log("Load test cancelled...")
    return
  }
  utils.saveToLowdb(err,result)
}

const loadTest = autocannon({
  ...utils.settings,
  title:"todo-actix-api",
  requests
},saveResults)

process.once('SIGINT',()=>{
  abort = true
  loadTest.stop()
})

autocannon.track(loadTest)