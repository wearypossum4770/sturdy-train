// const messageEvent = ({apiVersion, type, timeStamp, ...args}) => ({
//   timestamp: new Date().getTime(),
//   apiVersion,
//   event: type.toUpperCase(),
//   duration: timeStamp, 
//   ...args,
// })


// const ivsMessageEvent = ({data, type, timeStamp, target,}) => {
//   const [provider, id] =  data.split(":")
//   console.log(messageEvent({provider, id, apiVersion: window.usps.version,type, timeStamp,...parseClientInformation(target),...parsePerformance(target.performance) }))
// }
// const checkEngagement = () => {
//   const interval = setInterval(() => {
//     const start = sturdyTrainDataLayer.filter(layer=> layer[0].endsWith('visit') || layer[0].startsWith('visit'))[0].at(-1)
//     let now = Date.now();
//     if (start && now - start.timestamp > 9_000) {
//       sturdyTag({eventName: 'user_engagement'})  
//         console.log(now)
//         clearInterval(interval)    
//     }
//   }, 500);
// }

// const userEngagement = (event) => {
//   checkEngagement(event)
// }
const anyTrue = (...arr) => arr.flat().some(v=>v)
const allTrue = (...arr) => !arr.flat().some(v=> !v)
const isLegacyEvent = ({eventCategory,eventAction,eventLabel,eventValue,})=> anyTrue(eventCategory,eventAction,eventLabel,eventValue)
const isModernEvent = ({eventCategory,eventAction,eventLabel,eventValue}) => allTrue(eventCategory,eventAction,eventLabel,eventValue)
const sturdyTag = (args) => {
const {command = "", hitType = "", eventCategory = "", eventAction = "", eventLabel = "", eventValue = "", category ='', action ='', label ='', value =''} = args
return {
  command,
  hitType,
  isLegacyEvent: isLegacyEvent(args),
  isModernEvent: isModernEvent(args),
  category: eventCategory || category,
  action: eventAction || action,
  label: eventLabel || label,
  value: eventValue || value,

}
} 
console.log(isLegacyEvent({eventCategory: 'video', eventAction:''}))
console.log(isModernEvent({eventCategory: 'video', eventAction:''}))
console.log(isModernEvent({eventCategory: 'video', eventAction:'play', eventLabel: "Fall Campaign", eventValue: 42}))