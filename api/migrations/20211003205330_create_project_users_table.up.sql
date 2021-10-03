CREATE TABLE project_users (
    id SERIAL PRIMARY KEY,
    project_id INT NOT NULL,
    user_id INT NOT NULL,
    added_at DATE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
   CONSTRAINT fk_project
      FOREIGN KEY(project_id) 
	  REFERENCES projects(id),
    
   CONSTRAINT fk_user
      FOREIGN KEY(user_id) 
	  REFERENCES users(id)
);

CREATE UNIQUE INDEX project_user_idx ON project_users (project_id, user_id);

