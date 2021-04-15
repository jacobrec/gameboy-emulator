import { useForm } from 'react-hook-form';
import './FileSubmission.css';

function FileSubmission(props: any) {
    const { register, handleSubmit} = useForm();
  
    return (
      <div className="modal-box">
        <div className="modal-text">
          <h2>Load a Game</h2>
          <form onSubmit={handleSubmit(props.onSubmit)} className="form">
            <input required type="file" name="rom" ref={register}/>
            <button>Submit</button>
          </form>
        </div>
      </div>
    );
}

export default FileSubmission;