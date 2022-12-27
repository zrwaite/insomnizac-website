class UpdateProjectSkillIds < ActiveRecord::Migration[7.0]
  def change
    remove_column :projects, :skill_ids
  end
end
