import LabeledInput from './components/inputs/labeled-input'
import BaseInput from './components/inputs/base-input'
import IconArrowRight from './components/icons/icon-arrow-right'

function App() {
  return (
    <div className="App">
      <form action="#" method="POST">
        <BaseInput type='text' />
        <LabeledInput type='text' id={'test'}>Text Input</LabeledInput>
      </form>
    </div>
  )
}

export default App
