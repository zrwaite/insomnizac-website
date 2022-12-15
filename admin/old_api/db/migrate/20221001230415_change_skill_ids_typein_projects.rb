class ChangeSkillIdsTypeinProjects < ActiveRecord::Migration[7.0]
  def change
    remove_column :projects, :skills
    add_column :projects, :skill_ids, :string, array: true, default: []
  end
end
